pub(crate) mod commands;
mod error;
mod helper;
pub mod prompt;
mod runtime_handler;

use std::{
    mem::take,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
};

use helper::MyHelper;
use rush_lib::error::runtime::{Error, ErrorContext, InvalidSyscallReason, RuntimeError};
use rush_lib::runtime::{SYS13_OPEN, SYS14_READ, SYS15_WRITE, SYS16_CLOSE};
use rush_lib::{
    runtime::SteppedRuntime,
    Binary, RushError, Runtime,
};

use colored::*;
use commands::{Arguments, Command};
use rustyline::{
    config::Configurer, error::ReadlineError, At, Cmd, Editor, KeyCode, KeyEvent, Modifiers,
    Movement, Word,
};
use rush_lib::runtime::system_clock::get_curr_time_as_millis;

use rush_utils::RushConfig;

use self::error::{CommandError, CommandResult};

pub(crate) struct InteractiveState {
    pub(crate) config: Option<RushConfig>,
    pub(crate) commands: Vec<Command>,
    pub(crate) binary: Option<Binary>,
    pub(crate) runtime: Option<Runtime>,
    pub(crate) exited: bool,
    pub(crate) prev_command: Option<String>,
    pub(crate) confirm_exit: bool,
    pub(crate) interrupted: Arc<AtomicBool>,
}

impl InteractiveState {
    fn new() -> Self {
        Self {
            config: None,
            commands: vec![
                commands::load_command(),
                commands::run_command(),
                commands::step_command(),
                commands::reset_command(),
                commands::breakpoint_command(),
                commands::context_command(),
                commands::examine_command(),
                commands::print_command(),
                commands::help_command(),
                commands::exit_command(),
            ],
            binary: None,
            runtime: None,
            exited: false,
            prev_command: None,
            confirm_exit: false,
            interrupted: Arc::new(AtomicBool::new(false)),
        }
    }

    fn prompt(&self) -> &str {
        if self.confirm_exit {
            ""
        } else {
            "[rush] "
        }
    }

    fn cleanup_cmd(&mut self, cmd: String) {
        self.confirm_exit = false;
        self.prev_command = Some(cmd);
    }

    fn find_command(&self, cmd: &str) -> Option<Command> {
        self.commands
            .iter()
            .find(|command| command.name == cmd || command.aliases.iter().any(|alias| alias == cmd))
            .cloned()
    }

    fn do_exec(&mut self, line: &str) {
        let parts = match shlex::split(line) {
            Some(parts) => parts,
            None => return,
        };

        let command_name = match parts.first() {
            Some(command_name) => command_name,
            None => return,
        };

        let command = self.find_command(&command_name.to_ascii_lowercase());

        if command.is_none() {
            prompt::unknown_command(command_name);
            return;
        }

        let command = command.unwrap();
        let required = match &command.args {
            Arguments::Exactly {
                required,
                optional: _,
            } => required,
            Arguments::VarArgs {
                required,
                format: _,
            } => required,
        };

        if (parts.len() - 1) < required.len() {
            self.handle_error(
                CommandError::WithTip {
                    error: Box::new(CommandError::MissingArguments {
                        args: required.to_vec(),
                        instead: parts.to_vec(),
                    }),
                    tip: format!("try `{} {}`", "help".bold(), command_name.bold()),
                },
                true,
            );
            return;
        }

        let result = command.exec(self, command_name, &parts[1..]);
        match result {
            Ok(_) => {}
            Err(err) => self.handle_error(err, true),
        };
    }

    fn handle_error(&self, err: CommandError, nl: bool) {
        match err {
            CommandError::MissingArguments { args, instead } => {
                let mut err_msg = String::from("missing required parameter");

                if args.len() - (instead.len().saturating_sub(1)) > 1 {
                    err_msg.push('s');
                }

                err_msg.push(' ');

                err_msg.push_str(
                    &args[(instead.len().saturating_sub(1))..(args.len())]
                        .iter()
                        .map(|s| format!("{}{}{}", "<".magenta(), s.magenta(), ">".magenta()))
                        .collect::<Vec<String>>()
                        .join(" "),
                );

                prompt::error(err_msg);
            }
            CommandError::BadArgument { arg, instead } => {
                prompt::error(format!("bad argument `{}` for {}", instead, arg));
            }
            CommandError::ArgExpectedI32 { arg, instead } => {
                prompt::error(format!(
                    "parameter {} expected integer, got `{}` instead",
                    arg, instead
                ));
            }
            CommandError::ArgExpectedU32 { arg, instead } => {
                prompt::error(format!(
                    "parameter {} expected positive integer, got `{}` instead",
                    arg, instead
                ));
            }
            CommandError::InvalidBpId { arg } => {
                prompt::error(format!("breakpoint with id {} does not exist", arg.blue()));
            }
            CommandError::HelpUnknownCommand { command } => {
                prompt::error(format!("unknown command `{}`", command));
            }
            CommandError::CannotReadFile { path, os_error } => {
                prompt::error(format!("failed to read file `{}`: {}", path, os_error));
            }
            CommandError::CannotParseLine { line: _, error: _ } => {
                prompt::error("failed to parse");

                // self.rush_error(
                //     RushError::Parser(ParserError::new(
                //         parser::Error::ParseFailure,
                //         Rc::from(""),
                //         error.line,
                //         error.col as u32,
                //     )),
                //     ErrorContext::Repl,
                //     Some(line),
                // );
            }
            CommandError::CannotCompileLine { line, error } => {
                prompt::error("failed to compile instruction");
                self.rush_error(error, ErrorContext::Repl, Some(line));
            }
            CommandError::LineDoesNotExist { line_number } => {
                prompt::error(format!(
                    "line :{line_number} does not exist in this program"
                ));
            }
            CommandError::UnknownRegister { register } => {
                prompt::error(format!(
                    "unknown register: {}{}",
                    "$".yellow(),
                    register.bold()
                ));
            }
            CommandError::MustLoadFile => {
                prompt::error("you have to load a file first");
            }
            CommandError::MustSpecifyFile => {
                prompt::error(
                    "there are multiple files loaded, you must specify which file to use",
                );
            }
            CommandError::ProgramExited => {
                prompt::error("program has exited");
                prompt::tip(format!(
                    "try using `{}` or `{}`",
                    "back".bold(),
                    "reset".bold()
                ));
            }
            CommandError::CannotStepFurtherBack => prompt::error("can't step any further back"),
            CommandError::RuntimeError { rush_error } => {
                self.rush_error(rush_error, ErrorContext::Interactive, None);
            }
            CommandError::ReplRuntimeError { rush_error, line } => {
                self.rush_error(rush_error, ErrorContext::Repl, Some(line));
            }
            CommandError::WithTip { error, tip } => {
                self.handle_error(*error, false);
                prompt::tip(tip);
            }
            CommandError::UnknownLabel { label } => {
                prompt::error(format!("unknown label: \"{}\"", label));
            }
            CommandError::UninitialisedRegister { register } => {
                prompt::error(format!("register {register} is uninitialized"));
            }
            CommandError::UninitialisedPrint { addr } => {
                prompt::error(format!("memory at address 0x{:08x} is uninitialized", addr));
            }
            CommandError::UnterminatedString { good_parts } => {
                prompt::error(format!("unterminated string: \"{}\"", good_parts.red()));
                prompt::tip(format!(
                    "make sure your strings are null terminated - use {} instead of {}",
                    ".asciiz".green(),
                    ".ascii".red()
                ));
            }
        }

        if nl {
            println!();
        }
    }

    pub(crate) fn rush_error(
        &self,
        error: RushError,
        context: ErrorContext,
        _repl_line: Option<String>,
    ) {

        match error {
            RushError::Runtime(error) => error.show_error(
                context,
                self.runtime.as_ref().unwrap(),
            ),
        }
    }

    pub(crate) fn eval_stepped_runtime(
        &mut self,
        verbose: bool,
        result: Result<SteppedRuntime, (Runtime, RushError)>
    ) -> CommandResult<bool> {
        let mut breakpoint = false;
        let mut trapped = false;

        match result {
            Ok(Ok(new_runtime)) => {
                self.runtime = Some(new_runtime);
            }
            Ok(Err(guard)) => {
                // Ok(true) on exit or breakpoint, see self::exec_status
                use rush_lib::runtime::RuntimeSyscallGuard::*;

                match guard {
                    PrintInt(args, new_runtime) => {
                        self.runtime = Some(new_runtime);
                        runtime_handler::sys1_print_int(verbose, args.value);
                    }
                    PrintFloat(args, new_runtime) => {
                        self.runtime = Some(new_runtime);
                        runtime_handler::sys2_print_float(verbose, args.value);
                    }
                    PrintDouble(args, new_runtime) => {
                        self.runtime = Some(new_runtime);
                        runtime_handler::sys3_print_double(verbose, args.value);
                    }
                    PrintString(args, new_runtime) => {
                        self.runtime = Some(new_runtime);
                        runtime_handler::sys4_print_string(verbose, &args.value);
                    }
                    ReadInt(guard) => {
                        let value = runtime_handler::sys5_read_int(verbose);
                        self.runtime = Some(guard(value));
                    }
                    ReadFloat(guard) => {
                        let value = runtime_handler::sys6_read_float(verbose);
                        self.runtime = Some(guard(value));
                    }
                    ReadDouble(guard) => {
                        let value = runtime_handler::sys7_read_double(verbose);
                        self.runtime = Some(guard(value));
                    }
                    ReadString(args, guard) => {
                        let value = runtime_handler::sys8_read_string(verbose, args.max_len);
                        self.runtime = Some(guard(value));
                    }
                    Sbrk(args, new_runtime) => {
                        self.runtime = Some(new_runtime);
                        runtime_handler::sys9_sbrk(verbose, args.bytes);
                    }
                    Exit(new_runtime) => {
                        self.runtime = Some(new_runtime);
                        self.exited = true;

                        runtime_handler::sys10_exit(verbose);
                    }
                    PrintChar(args, new_runtime) => {
                        self.runtime = Some(new_runtime);
                        runtime_handler::sys11_print_char(verbose, args.value);
                    }
                    ReadChar(guard) => {
                        let value = runtime_handler::sys12_read_char(verbose);
                        self.runtime = Some(guard(value));
                    }
                    Open(_args, guard) => {

                        let new_runtime = guard(-1);
                        self.runtime = Some(new_runtime);
                        return Err(CommandError::RuntimeError {
                            rush_error: RushError::Runtime(RuntimeError::new(
                                Error::InvalidSyscall {
                                    syscall: SYS13_OPEN,
                                    reason: InvalidSyscallReason::Unimplemented,
                                },
                            )),
                        });

                        // let value = runtime_handler::sys13_open(verbose, args);
                        // self.runtime = Some(guard(value));
                    }
                    Read(_args, guard) => {

                        let new_runtime = guard((-1, Vec::new()));
                        self.runtime = Some(new_runtime);
                        return Err(CommandError::RuntimeError {
                            rush_error: RushError::Runtime(RuntimeError::new(
                                Error::InvalidSyscall {
                                    syscall: SYS14_READ,
                                    reason: InvalidSyscallReason::Unimplemented,
                                },
                            )),
                        });

                        // let value = runtime_handler::sys14_read(verbose, args);
                        // self.runtime = Some(guard(value));
                    }
                    Write(_args, guard) => {

                        let new_runtime = guard(-1);
                        self.runtime = Some(new_runtime);
                        return Err(CommandError::RuntimeError {
                            rush_error: RushError::Runtime(RuntimeError::new(
                                Error::InvalidSyscall {
                                    syscall: SYS15_WRITE,
                                    reason: InvalidSyscallReason::Unimplemented,
                                },
                            )),
                        });

                        // let value = runtime_handler::sys15_write(verbose, args);
                        // self.runtime = Some(guard(value));
                    }
                    Close(_args, guard) => {

                        let new_runtime = guard(-1);
                        self.runtime = Some(new_runtime);
                        return Err(CommandError::RuntimeError {
                            rush_error: RushError::Runtime(RuntimeError::new(
                                Error::InvalidSyscall {
                                    syscall: SYS16_CLOSE,
                                    reason: InvalidSyscallReason::Unimplemented,
                                },
                            )),
                        });

                        // let value = runtime_handler::sys16_close(verbose, args);
                        // self.runtime = Some(guard(value));
                    }
                    ExitStatus(args, new_runtime) => {
                        self.runtime = Some(new_runtime);
                        self.exited = true;

                        runtime_handler::sys17_exit_status(verbose, args.exit_code);
                    }
                    Breakpoint(new_runtime) => {
                        self.runtime = Some(new_runtime);
                        breakpoint = true;
                    }
                    Trap(new_runtime) => {
                        self.runtime = Some(new_runtime);
                        runtime_handler::trap(verbose);
                        trapped = true;
                    }
                }
            }
            Err((new_runtime, err)) => {
                self.runtime = Some(new_runtime);

                self.runtime.as_mut().unwrap().system_clock.stop_time = get_curr_time_as_millis();

                let delta_time_sec = (self.runtime.as_mut().unwrap().system_clock.stop_time - self.runtime.as_mut().unwrap().system_clock.start_time) / 1000 + 1;
                let ips = self.runtime.as_mut().unwrap().system_clock.steps / delta_time_sec;
                println!("Emulation running on {} sec., IPS = {}", delta_time_sec, ips);

                return Err(CommandError::RuntimeError { rush_error: err });
            }
        };

        Ok(if self.exited {
            true
        } else {
            false
        })
    }

    pub(crate) fn step(&mut self, verbose: bool) -> CommandResult<bool> {
        let runtime = take(self.runtime.as_mut().unwrap());
        self.eval_stepped_runtime(verbose, runtime.step())
    }

    pub(crate) fn run(&mut self) -> CommandResult<String> {
        if self.exited {
            return Err(CommandError::ProgramExited);
        }

        self.interrupted.store(false, Ordering::SeqCst);
        self.runtime.as_mut().unwrap().system_clock.start_time = get_curr_time_as_millis();
        while !self.interrupted.load(Ordering::SeqCst) {
            if self.step(false)? {
                break;
            }
            self.runtime.as_mut().unwrap().system_clock.steps += 1;
            self.runtime.as_mut().unwrap().system_clock.update(1)
        }

        Ok("".into())
    }

    pub(crate) fn reset(&mut self) -> CommandResult<()> {
        self.runtime.as_ref().unwrap().reset();
        self.exited = false;

        Ok(())
    }

    fn exec_command(&mut self, line: String) {
        self.do_exec(&line);
        self.cleanup_cmd(line);
    }

    fn exec_prev(&mut self) {
        if let Some(cmd) = self.prev_command.take() {
            self.exec_command(cmd);
        }
    }
}

pub(crate) fn editor_init() -> Editor<MyHelper> {
    let mut rl = Editor::new().unwrap();

    rl.set_check_cursor_position(true);

    let helper = MyHelper::new();
    rl.set_helper(Some(helper));

    rl.bind_sequence(
        KeyEvent(KeyCode::Left, Modifiers::CTRL),
        Cmd::Move(Movement::BackwardWord(1, Word::Emacs)),
    );
    rl.bind_sequence(
        KeyEvent(KeyCode::Right, Modifiers::CTRL),
        Cmd::Move(Movement::ForwardWord(1, At::BeforeEnd, Word::Emacs)),
    );

    rl
}

pub fn launch() -> ! {
    let mut rl = editor_init();
    let mut interactive_state = InteractiveState::new();
    let interrupted = interactive_state.interrupted.clone();
    ctrlc::set_handler(move || interrupted.store(true, Ordering::SeqCst))
        .expect("Failed to set signal handler!");

    loop {
        let readline = rl.readline(interactive_state.prompt());

        match readline {
            Ok(line) => {
                if line.is_empty() {
                    if !interactive_state.confirm_exit {
                        interactive_state.exec_prev();
                    }

                    interactive_state.confirm_exit = false;
                    continue;
                }

                rl.add_history_entry(&line);
                interactive_state.exec_command(line);
            }
            Err(ReadlineError::Interrupted) => {}
            Err(ReadlineError::Eof) => {
                std::process::exit(0);
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }

    std::process::exit(0)
}
