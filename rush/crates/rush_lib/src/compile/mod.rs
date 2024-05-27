use crate::{
    util::Safe,
};
use serde::{Deserialize, Serialize};
use std::fs;

mod bytes;

mod breakpoints;

use rush_utils::RushConfig;

pub const TEXT_BOT: u32 = 0x00400000;
pub const TEXT_TOP: u32 = 0x0040C158;
pub const GLOBAL_BOT: u32 = 0x0040C158;
pub const GLOBAL_PTR: u32 = 0x10008000;
pub const DATA_BOT: u32 = 0x0040C158;
pub const HEAP_BOT: u32 = 0x10040000;
pub const STACK_BOT: u32 = 0x7FFF0000;
pub const STACK_PTR: u32 = 0x7FFFFFF0;
pub const STACK_TOP: u32 = 0x7FFFFFFF;
pub const KTEXT_BOT: u32 = 0x80000000;
pub const KDATA_BOT: u32 = 0x90000000;
pub const PRINTF_ADDR: u32 = 0x0040035C;

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct Binary {
    pub text: Vec<Safe<u8>>,
    pub data: Vec<Safe<u8>>,
}

impl Binary {
    pub fn new(rush_config: &RushConfig) -> Self {
        let binary_content: Vec<Safe<u8>> = fs::read(
            rush_config.executable.clone()
        ).unwrap().clone().into_iter().map(Safe::Valid).collect();
        let text_size = rush_config.memory.text.end - rush_config.memory.text.start;
        let segments = binary_content.split_at(text_size as usize);
        Self {
            text: segments.0.to_vec(),
            data: segments.1.to_vec()

        }
    }
}
