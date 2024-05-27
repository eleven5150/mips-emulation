use std::sync::atomic::Ordering;
use std::vec;

use crate::interactive::error::CommandError;

use super::Command;
use super::*;
use colored::*;

pub(crate) fn step_command() -> Command {
    command(
        "step",
        vec!["s"],
        vec![],
        vec!["times"],
        vec![],
        "step forwards or execute a subcommand",
        |cmd, state, label, args| {
            if label == "__help__" && args.is_empty() {
                return Ok(get_long_help());
            }

            let cmd = args.get(0).and_then(|arg| {
                cmd.subcommands
                    .iter()
                    .find(|c| &c.name == arg || c.aliases.contains(arg))
            });
            match cmd {
                Some(cmd) => cmd.exec(state, label, &args[1..]),
                None => step_forward(state, label, args),
            }
        },
    )
}

fn get_long_help() -> String {
    format!(
        "A collection of commands for {3}ping through the program. Available {0}s are:\n\
         \n\
         {3} {4}    : steps backwards instead of forwards\n\
         {3} {5} : steps forwards until the next syscall\n\
         \n\
         {6} {7} will provide more information about the specified subcommand.\n\
         \n\
         By default, this steps forwards one instruction, or {1} instructions if specified.\n\
         This will run in \"verbose\" mode, printing out the instruction that was\n\
         \x20 executed, and verbosely printing any system calls that are executed.\n\
         To step backwards (i.e. back in time), use `{2}`.",
        "[subcommand]".magenta(),
        "[times]".magenta(),
        "back".bold(),
        "step".yellow().bold(),
        "back".purple(),
        "syscall".purple(),
        "help step".white().bold(),
        "[subcommand]".magenta().bold(),
    )
}

fn step_forward(state: &mut InteractiveState, label: &str, args: &[String]) -> Result<String, CommandError> {
    let times = match args.first() {
        Some(arg) => match arg.parse::<u32>() {
            Ok(num) => Ok(num),
            Err(_) => Err(CommandError::WithTip {
                error: Box::new(CommandError::ArgExpectedI32 {
                    arg: "[times]".bright_magenta().to_string(),
                    instead: arg.to_owned(),
                }),
                tip: format!("try `{} {}`", "help".bold(), label.bold()),
            }),
        },
        None => Ok(1),
    }?;

    if state.exited {
        return Err(CommandError::ProgramExited);
    }

    state.interrupted.store(false, Ordering::SeqCst);
    for _ in 0..times {
        let runtime = state.runtime.as_ref().unwrap();

        if let Ok(inst) = runtime.current_inst() {
            util::print_inst(
                inst,
                runtime.state().pc()
            );
        }

        let step = state.step(true)?;

        if step | state.interrupted.load(Ordering::SeqCst) {
            break;
        }
    }

    Ok("".into())
}
