use serde::{Deserialize, Serialize};
use std::fmt::Display;

use crate::{inst::instruction::Signature};
use rush_parser::{MpDirective, MpInstruction};

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub enum Error {
    NumberedRegisterOutOfRange {
        reg_num: i32,
    },
    NamedRegisterOutOfRange {
        reg_name: char,
        reg_index: i32,
    },
    UnknownRegister {
        reg_name: String,
    },

    UnknownInstruction {
        inst_ast: MpInstruction,
    },
    InstructionBadFormat {
        inst_ast: MpInstruction,
        correct_formats: Vec<Signature>,
    },
    InstructionSimName {
        inst_ast: MpInstruction,
        similar_instns: Vec<Signature>,
    },

    RedefinedLabel {
        label: String,
    },
    UnresolvedLabel {
        label: String,
        similar: Vec<String>,
    },

    RedefinedConstant {
        label: String,
    },
    UnresolvedConstant {
        label: String,
    },

    ConstantValueDoesNotFit {
        directive_type: DirectiveType,
        value: i64,
        range_low: i64,
        range_high: i64,
    },

    DataInTextSegment {
        directive_type: MpDirective,
    },
    InstructionInDataSegment,

    TooMuchData {
        data_size: u32,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum DirectiveType {
    Byte,
    Half,
    Word,
    Align,
    Space,
}

impl Display for DirectiveType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Byte => "byte",
                Self::Half => "half",
                Self::Word => "word",
                Self::Align => "align",
                Self::Space => "space",
            }
        )
    }
}
