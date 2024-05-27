use crate::interactive::prompt;

use super::*;
use colored::*;
use rush_lib::{Binary, Runtime};
use rush_utils::RushConfig;

pub(crate) fn load_command() -> Command {
    command(
        "load",
        vec!["l"],
        vec!["config"],
        vec![],
        vec![],
        "load a config file to run",
        |_, inter_state, label, args| {
            if label == "__help__" {
                return Ok(
                    format!(
                        "Loads a config file to run, overwriting whatever is currently loaded.\n\
                         This command must be run prior to many others, such as `{}`, `{}`, `{}`, ...",
                        "run".bold(),
                        "step".bold(),
                        "print".bold(),
                    ),
                );
            }

            let config_file = &args[0];

            inter_state.config = Some(RushConfig::new(config_file));

            inter_state.binary = Some(Binary::new(&inter_state.config.clone().unwrap()));

            inter_state.runtime = Some(Runtime::new(&inter_state.binary.clone().unwrap(), inter_state.config.clone().unwrap()));

            inter_state.exited = false;

            prompt::success_nl("file loaded");

            Ok("".into())
        },
    )
}
