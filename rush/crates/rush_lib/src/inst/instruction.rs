use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct InstSignature {
    name: String,
    compile: CompileSignature,
    runtime: RuntimeSignature,
    runtime_meta: RuntimeMetadata,
    meta: InstMetadata,
}

impl InstSignature {
    pub fn new(
        name: String,
        compile: CompileSignature,
        runtime: RuntimeSignature,
        runtime_meta: RuntimeMetadata,
        meta: InstMetadata,
    ) -> Self {
        Self {
            name,
            compile,
            runtime,
            runtime_meta,
            meta,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct CompileSignature {
    format: Vec<ArgumentType>,
    relative_label: bool,
}

impl CompileSignature {
    pub fn new(format: Vec<ArgumentType>, relative_label: bool) -> Self {
        Self {
            format,
            relative_label,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum ArgumentType {
    Rd,
    Rs,
    Rt,
    Shamt,
    I16,
    U16,
    J,
    OffRs,
    OffRt,
    F32,
    F64,

    // pseudo
    I32,
    U32,
    Off32Rs,
    Off32Rt,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum RuntimeSignature {
    R {
        opcode: u8,
        funct: u8,
        shamt: Option<u8>,
        rs: Option<u8>,
        rt: Option<u8>,
        rd: Option<u8>,
    },
    I {
        opcode: u8,
        rt: Option<u8>,
    },
    J {
        opcode: u8,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct RuntimeMetadata {
    reads: Vec<ReadsRegisterType>,
}

impl RuntimeMetadata {
    pub fn new(reads: Vec<ReadsRegisterType>) -> Self {
        Self { reads }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum ReadsRegisterType {
    Rs,
    Rt,
    OffRs,
    OffRt,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct InstMetadata {
    desc_short: Option<String>,
    desc_long: Option<String>,
}

impl InstMetadata {
    pub fn new(desc_short: Option<String>, desc_long: Option<String>) -> Self {
        Self {
            desc_short,
            desc_long,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct PseudoSignature {
    name: String,
    compile: CompileSignature,
    expand: Vec<PseudoExpand>,
}

impl PseudoSignature {
    pub fn new(name: String, compile: CompileSignature, expand: Vec<PseudoExpand>) -> Self {
        Self {
            name,
            compile,
            expand,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct PseudoExpand {
    inst: String,
    data: Vec<String>,
}

impl PseudoExpand {
    pub fn new(inst: String, data: Vec<String>) -> Self {
        Self { inst, data }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum Signature {
    Native(InstSignature),
    Pseudo(PseudoSignature),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
pub enum SignatureRef<'a> {
    Native(&'a InstSignature),
    Pseudo(&'a PseudoSignature),
}

impl fmt::Display for ArgumentType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ArgumentType::Rd => write!(f, "$Rd"),
            ArgumentType::Rs => write!(f, "$Rs"),
            ArgumentType::Rt => write!(f, "$Rt"),
            ArgumentType::Shamt => write!(f, "shift"),
            ArgumentType::I16 => write!(f, "i16"),
            ArgumentType::U16 => write!(f, "u16"),
            ArgumentType::J => write!(f, "label"),
            ArgumentType::OffRs => write!(f, "i16($Rs)"),
            ArgumentType::OffRt => write!(f, "i16($Rt)"),
            ArgumentType::F32 => write!(f, "f32"),
            ArgumentType::F64 => write!(f, "f64"),
            ArgumentType::I32 => write!(f, "i32"),
            ArgumentType::U32 => write!(f, "u32"),
            ArgumentType::Off32Rs => write!(f, "i32($Rs)"),
            ArgumentType::Off32Rt => write!(f, "i32($Rt)"),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum PseudoVariable {
    Rd,
    Rs,
    Rt,
    Shamt,
    I16,
    U16,
    J,
    OffRs,
    OffRt,
    F32,
    F64,
    Off,

    // pseudo
    I32,
    U32,
    Off32,
}

impl PseudoVariable {
    fn name(&self) -> String {
        match self {
            Self::Rd => "rd",
            Self::Rs => "rs",
            Self::Rt => "rt",
            Self::Shamt => "shamt",
            Self::I16 => "i16",
            Self::U16 => "u16",
            Self::J => "j",
            Self::OffRs => "offrs",
            Self::OffRt => "offrt",
            Self::F32 => "f32",
            Self::F64 => "f64",
            Self::Off => "off",

            // pseudo
            Self::I32 => "i32",
            Self::U32 => "u32",
            Self::Off32 => "off32",
        }
        .to_string()
    }
}
