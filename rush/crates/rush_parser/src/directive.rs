use std::fmt::Display;

use crate::{
    constant::{MpConstValueLoc},
    parser::Position,
};

use serde::{Deserialize, Serialize};

pub type MpDirectiveLoc = (MpDirective, Position);

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub enum MpDirective {
    Text,
    Data,
    Ascii(String),
    Asciiz(String),
    Byte(Vec<(MpConstValueLoc, Option<MpConstValueLoc>)>),
    Half(Vec<(MpConstValueLoc, Option<MpConstValueLoc>)>),
    Word(Vec<(MpConstValueLoc, Option<MpConstValueLoc>)>),
    Float(Vec<(f32, Option<MpConstValueLoc>)>),
    Double(Vec<(f64, Option<MpConstValueLoc>)>),
    Align(MpConstValueLoc),
    Space(MpConstValueLoc),
    Globl(String),
}

impl Display for MpDirective {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use MpDirective::*;

        write!(
            f,
            "{}",
            match self {
                Ascii(_) => "ascii",
                Asciiz(_) => "asciiz",
                Byte(_) => "byte",
                Half(_) => "half",
                Word(_) => "word",
                Float(_) => "float",
                Double(_) => "double",
                Align(_) => "align",
                Space(_) => "space",
                Globl(_) => "globl",
                Text => "text",
                Data => "data",
            }
        )
    }
}
