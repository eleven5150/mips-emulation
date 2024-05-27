use std::fmt;

use crate::{
    number::{parse_number, MpNumber},
    register::{parse_register, MpRegister},
    Span,
};
use nom::{
    branch::alt,
    combinator::map,
    sequence::tuple,
    IResult,
};
use nom_locate::position;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct MpInstruction {
    pub(crate) name: String,
    pub(crate) arguments: Vec<(MpArgument, u32, u32)>,
    pub(crate) line: u32,
    pub(crate) col: u32,
    pub(crate) col_end: u32,
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub enum MpArgument {
    Register(MpRegister),
    Number(MpNumber),
}

impl MpInstruction {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn arguments(&self) -> &[(MpArgument, u32, u32)] {
        &self.arguments
    }

    pub fn arguments_mut(&mut self) -> &mut Vec<(MpArgument, u32, u32)> {
        &mut self.arguments
    }

    pub fn line(&self) -> u32 {
        self.line
    }

    pub fn col(&self) -> u32 {
        self.col
    }

    pub fn col_end(&self) -> u32 {
        self.col_end
    }
}

impl fmt::Display for MpArgument {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Register(reg) => write!(f, "{}", reg),
            Self::Number(num) => write!(f, "{}", num),
            // Self::LabelPlusConst(label, the_const) => write!(f, "{} + {}", label, the_const),
        }
    }
}

pub fn parse_argument(i: Span<'_>) -> IResult<Span<'_>, (MpArgument, u32, u32)> {
    map(
        tuple((
            position,
            alt((parse_argument_reg, parse_argument_num)),
            position,
        )),
        |(pos, arg, pos_end)| (arg, pos.get_column() as u32, pos_end.get_column() as u32),
    )(i)
}

fn parse_argument_reg(i: Span<'_>) -> IResult<Span<'_>, MpArgument> {
    map(parse_register, MpArgument::Register)(i)
}

fn parse_argument_num(i: Span<'_>) -> IResult<Span<'_>, MpArgument> {
    map(parse_number, MpArgument::Number)(i)
}
