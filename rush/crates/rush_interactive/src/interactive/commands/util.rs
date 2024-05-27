use crate::interactive::{
    error::{CommandError, CommandResult},
};
use colored::*;

pub(crate) fn expect_u32<F>(
    command: &str,
    name: &str,
    arg: &str,
    neg_tip: Option<F>,
) -> CommandResult<u32>
    where
        F: Fn(i32) -> String,
{
    match arg.parse::<u32>() {
        Ok(num) => Ok(num),
        Err(_) => Err({
            let err = CommandError::ArgExpectedU32 {
                arg: name.to_string(),
                instead: arg.to_string(),
            };

            match (arg.parse::<i32>(), neg_tip) {
                (Ok(neg), Some(f)) => CommandError::WithTip {
                    error: Box::new(err),
                    tip: f(neg),
                },
                _ => CommandError::WithTip {
                    error: Box::new(err),
                    tip: format!("try `{} {}`", "help".bold(), command.bold()),
                },
            }
        }),
    }
}

pub(crate) fn print_inst(
    inst: u32,
    addr: u32,
) {
    let bytes = inst.to_be_bytes();
    println!("{:#X}: {:02X} {:02X} {:02X} {:02X}", addr, bytes[0], bytes[1], bytes[2], bytes[3]);
}