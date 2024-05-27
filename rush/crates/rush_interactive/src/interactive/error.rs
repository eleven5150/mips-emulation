use rush_lib::{RushError, Register};
use rush_parser::ErrorLocation;

pub type CommandResult<T> = Result<T, CommandError>;

#[derive(Debug)]
#[allow(dead_code)]
pub enum CommandError {
    BadArgument {
        arg: String,
        instead: String,
    },
    MissingArguments {
        args: Vec<String>,
        instead: Vec<String>,
    },
    ArgExpectedI32 {
        arg: String,
        instead: String,
    },
    ArgExpectedU32 {
        arg: String,
        instead: String,
    },
    InvalidBpId {
        arg: String,
    },
    HelpUnknownCommand {
        command: String,
    },
    CannotReadFile {
        path: String,
        os_error: String,
    },
    CannotParseLine {
        line: String,
        error: ErrorLocation,
    },
    CannotCompileLine {
        line: String,
        error: RushError,
    },
    LineDoesNotExist {
        line_number: u32,
    },
    UnknownRegister {
        register: String,
    },
    UnknownLabel {
        label: String,
    },
    UninitialisedRegister {
        register: Register,
    },
    UninitialisedPrint {
        addr: u32,
    },
    UnterminatedString {
        good_parts: String,
    },

    MustLoadFile,
    MustSpecifyFile,
    ProgramExited,

    CannotStepFurtherBack,
    RuntimeError {
        rush_error: RushError,
    },
    ReplRuntimeError {
        rush_error: RushError,
        line: String,
    },

    WithTip {
        error: Box<CommandError>,
        tip: String,
    },
}
