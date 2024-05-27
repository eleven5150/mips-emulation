use std::{
    fmt::{Debug, Display},
    io::Write,
    process,
    str::FromStr,
};

use clap::Parser;
use rush_lib::error::runtime::ErrorContext;
use rush_lib::{
    Binary, RushError, Runtime
};
use rush_utils::RushConfig;
use text_io::try_read;
use rush_lib::runtime::system_clock::get_curr_time_as_millis;

#[derive(Parser, Debug)]
struct Opts {
    /// Emulation config with binaries and memory mapping
    #[arg()]
    config: Option<String>
}

fn main() {
    let args: Opts = Opts::parse();

    if args.config.is_none() {
        // launch() returns !
        rush_interactive::launch();
    }

    let config = RushConfig::new(&args.config.unwrap());

    let binary = Binary::new(&config);

    let mut runtime = Runtime::new(&binary, config);

    runtime.system_clock.start_time = get_curr_time_as_millis();
    loop {
        match runtime.step() {
            Ok(stepped_runtime) => {
                match stepped_runtime {
                    Ok(new_runtime) => {
                        runtime = new_runtime;
                    }
                    Err(runtime_guard) => {
                        use rush_lib::runtime::RuntimeSyscallGuard::*;

                        match runtime_guard {
                            PrintInt(args, new_runtime) => {
                                print!("{}", args.value);
                                std::io::stdout().flush().unwrap();

                                runtime = new_runtime;
                            }
                            PrintFloat(args, new_runtime) => {
                                print!("{}", args.value);
                                std::io::stdout().flush().unwrap();

                                runtime = new_runtime;
                            }
                            PrintDouble(args, new_runtime) => {
                                print!("{}", args.value);
                                std::io::stdout().flush().unwrap();

                                runtime = new_runtime;
                            }
                            PrintString(args, new_runtime) => {
                                print!("{}", String::from_utf8_lossy(&args.value));
                                std::io::stdout().flush().unwrap();

                                runtime = new_runtime;
                            }
                            ReadInt(guard) => {
                                let number = get_input_int("int").unwrap_or(0);
                                runtime = guard(number);
                            }
                            ReadFloat(guard) => {
                                let number = get_input_eof("float").unwrap_or(0.0);
                                runtime = guard(number);
                            }
                            ReadDouble(guard) => {
                                let number = get_input_eof("double").unwrap_or(0.0);
                                runtime = guard(number);
                            }
                            ReadString(args, guard) => {
                                let string = read_string(args.max_len);
                                runtime = guard(string.into_bytes());
                            }
                            Sbrk(_args, new_runtime) => {
                                runtime = new_runtime;
                            }
                            Exit(_new_runtime) => {
                                process::exit(0);
                            }
                            PrintChar(args, new_runtime) => {
                                print!("{}", args.value as char);
                                std::io::stdout().flush().unwrap();

                                runtime = new_runtime;
                            }
                            ReadChar(guard) => {
                                let character: char = get_input_eof("character").unwrap_or('\0');
                                runtime = guard(character as u8);
                            }
                            Open(_args, _guard) => {
                                println!("open");
                                process::exit(1);
                            }
                            Read(_args, _guard) => {
                                println!("read");
                                process::exit(1);
                            }
                            Write(_args, _guard) => {
                                println!("write");
                                process::exit(1);
                            }
                            Close(_args, _guard) => {
                                println!("close");
                                process::exit(1);
                            }
                            ExitStatus(args, _new_runtime) => {
                                process::exit(args.exit_code);
                            }
                            Breakpoint(new_runtime) => {
                                runtime = new_runtime;
                            }
                            Trap(new_runtime) => {
                                runtime = new_runtime;
                            }
                        }
                    }
                }
            }
            Err((old_runtime, RushError::Runtime(err))) => {
                runtime = old_runtime;
                runtime.system_clock.stop_time = get_curr_time_as_millis();

                let delta_time_sec = (runtime.system_clock.stop_time - runtime.system_clock.start_time) / 1000 + 1;
                let ips = runtime.system_clock.steps / delta_time_sec;
                println!("Emulation running on {} sec., IPS = {}", delta_time_sec, ips);

                println!();
                err.show_error(
                    ErrorContext::Binary,
                    &runtime,
                );

                process::exit(1);
            }
        }
        runtime.system_clock.steps += 1;
        runtime.system_clock.update(1)
    }
}

fn get_input<T>(name: &str, line: bool) -> T
    where
        T: FromStr + Display,
        <T as FromStr>::Err: Debug,
{
    loop {
        let result: Result<T, _> = if line {
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();

            input.parse().map_err(|_| ())
        } else {
            try_read!().map_err(|_| ())
        };

        match result {
            Ok(n) => return n,
            Err(_) => {
                print!("[rush] bad input (expected {}), try again: ", name);
                std::io::stdout().flush().unwrap();

                continue;
            }
        };
    }
}

fn get_input_eof<T>(name: &str) -> Option<T>
    where
        T: FromStr + Display,
        <T as FromStr>::Err: Debug,
{
    loop {
        let result: Result<T, _> = try_read!();

        match result {
            Ok(n) => return Some(n),
            Err(text_io::Error::Parse(leftover, _)) => {
                if leftover.is_empty() {
                    return None;
                }

                print!("[rush] bad input (expected {}), try again: ", name);
                std::io::stdout().flush().unwrap();
                continue;
            }
            Err(_) => {
                print!("[rush] bad input (expected {}), try again: ", name);
                std::io::stdout().flush().unwrap();
                continue;
            }
        };
    }
}

fn get_input_int(name: &str) -> Option<i32> {
    loop {
        let result: Result<i128, _> = try_read!();

        match result {
            Ok(n) => match i32::try_from(n) {
                Ok(n) => return Some(n),
                Err(_) => {
                    println!("[rush] bad input (too big to fit in 32 bits)");
                    println!(
                        "[rush] if you want the value to be truncated to 32 bits, try {}",
                        n as i32
                    );
                    print!("[rush] try again: ");
                    std::io::stdout().flush().unwrap();
                    continue;
                }
            },
            Err(text_io::Error::Parse(leftover, _)) => {
                if leftover.is_empty() {
                    return None;
                }

                print!("[rush] bad input (expected {}), try again: ", name);
                std::io::stdout().flush().unwrap();
                continue;
            }
            Err(_) => {
                print!("[rush] bad input (expected {}), try again: ", name);
                std::io::stdout().flush().unwrap();
                continue;
            }
        };
    }
}

fn read_string(_max_len: u32) -> String {
    loop {
        let input: String = get_input("string", true);

        // if input.len() > max_len as usize {
        //     println!("[rush] bad input (max string length specified as {}, given string is {} bytes)", max_len, input.len());
        //     print!  ("[rush] please try again: ");
        //     std::io::stdout().flush().unwrap();

        //     continue;
        // }

        // if input.len() == max_len as usize {
        //     println!("[rush] bad input (max string length specified as {}, given string is {} bytes -- must be at least one byte fewer, for NULL character), try again: ", max_len, input.len());
        //     print!  ("[rush] please try again: ");
        //     std::io::stdout().flush().unwrap();

        //     continue;
        // }

        return input;
    }
}
