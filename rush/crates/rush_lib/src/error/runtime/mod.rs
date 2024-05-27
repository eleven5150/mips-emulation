use crate::{
    util::Segment,
    Register, Runtime,
};
use colored::Colorize;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct RuntimeError {
    error: Error,
}

impl RuntimeError {
    pub fn new(error: Error) -> Self {
        Self { error }
    }

    pub fn error(&self) -> &Error {
        &self.error
    }

    pub fn show_error(
        &self,
        context: ErrorContext,
        runtime: &Runtime,
    ) {
        println!(
            "{}{} {}",
            "error".bright_red().bold(),
            ":".bold(),
            self.error
                .message(context, runtime)
        );
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum ErrorContext {
    Binary,
    Interactive,
    Repl,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum Error {
    UnknownInstruction {
        addr: u32,
    },
    Uninitialised {
        value: Uninitialised,
    },
    UnalignedAccess {
        addr: u32,
        alignment_requirement: AlignmentRequirement,
    },

    IntegerOverflow,
    DivisionByZero,

    SegmentationFault {
        addr: u32,
        access: SegmentationFaultAccessType,
    },
    InvalidSyscall {
        syscall: i32,
        reason: InvalidSyscallReason,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum Uninitialised {
    Byte { addr: u32 },
    Half { addr: u32 },
    Word { addr: u32 },
    Register { reg_num: u32 },
    Lo,
    Hi,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum AlignmentRequirement {
    Half,
    Word,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum InvalidSyscallReason {
    Unimplemented, // Invalid because we don't have an implementation for it but it does exist
    Unknown,       // Invalid because it doesn't exist to begin with
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum SegmentationFaultAccessType {
    Read,
    Write,
    Execute,
}

impl Error {
    pub fn message(
        &self,
        context: ErrorContext,
        runtime: &Runtime,
    ) -> String {
        match self {
            Error::UnknownInstruction { addr } => {
                let message = "could not find instruction at";
                let zero_x = "0x".yellow();

                format!("{} {}{:08x}\n", message, zero_x, addr)
            }

            Error::Uninitialised { value } => {
                let name = match value {
                    Uninitialised::Byte { addr }
                    | Uninitialised::Half { addr }
                    | Uninitialised::Word { addr } => {
                        let size = match value {
                            Uninitialised::Byte { addr: _ } => "byte",
                            Uninitialised::Half { addr: _ } => "half",
                            Uninitialised::Word { addr: _ } => "word",
                            _ => unreachable!(),
                        };

                        let message = "is uninitialised";
                        let zero_x = "0x".yellow();
                        return format!("{} at {}{:08x} {}", size, zero_x, addr, message);
                    }

                    Uninitialised::Register { reg_num } => {
                        let name = Register::from_u32(*reg_num).unwrap().to_lower_str();

                        name
                    }

                    Uninitialised::Lo => {
                        let name = "lo";

                        name
                    }

                    Uninitialised::Hi => {
                        let name = "hi";

                        name
                    }
                };

                let mut error = String::new();
                error.push_str("program tried to read an uninitialised register\n");

                let inst = runtime.state.read_mem_pc().unwrap();

                if let ErrorContext::Binary | ErrorContext::Interactive = context {
                    error.push_str("the instruction that failed was:\n");
                    error.push_str(&format!("{:#X}", inst));
                    error.push('\n');
                }

                error.push_str(&format!(
                    "\nthis happened because {}{} was uninitialised.\n",
                    "$".yellow(),
                    name.bold()
                ));


                error.push('\n');

                error
            }

            Error::UnalignedAccess {
                addr,
                alignment_requirement,
            } => {
                let mut error = String::new();

                error.push_str("unaligned access\n");

                let inst = runtime.state.read_mem_pc().unwrap();

                let alignment_bytes = match alignment_requirement {
                    AlignmentRequirement::Half => 2,
                    AlignmentRequirement::Word => 4,
                };

                if let ErrorContext::Binary | ErrorContext::Interactive = context {
                    error.push_str("\nerror at address:\n");
                    error.push_str(&format!("{:#X}", addr));
                    error.push_str("\nthe instruction that failed was:\n");
                    error.push_str(&format!("{:X}", inst));
                    error.push_str("\nalignment must be:\n");
                    error.push_str(&format!("{}", alignment_bytes));
                    error.push('\n');
                }

                error.push('\n');
                error
            }

            Error::IntegerOverflow => {
                let mut error = String::new();
                error.push_str("integer overflow\n");

                let inst = runtime.state.read_mem_pc().unwrap();

                if let ErrorContext::Binary | ErrorContext::Interactive = context {
                    error.push_str("\nthe instruction that failed was:\n");
                    error.push_str(&format!("{:#X}", inst));
                    error.push('\n');
                }

                let rs = (inst >> 21) & 0x1F;
                let rs_value = runtime.state.read_register(rs).unwrap();
                error.push_str("values:\n");
                error.push_str(&format!(
                    " - {}{} = {}\n",
                    "$".yellow(),
                    Register::from_u32(rs).unwrap().to_lower_str().bold(),
                    rs_value,
                ));

                error
            }

            Error::DivisionByZero => {
                let mut error = String::new();

                error.push_str("division by zero\n");

                let inst = runtime.state.read_mem_word(runtime.state.pc()).unwrap();

                if let ErrorContext::Binary | ErrorContext::Interactive = context {
                    error.push_str("\nthe instruction that failed was:\n");
                    error.push_str(&format!("{:#X}", inst));
                    error.push('\n');
                }

                let rs = (inst >> 21) & 0x1F;
                let rt = (inst >> 16) & 0x1F;

                error.push_str("\nvalues:\n");

                error.push_str(&format!(
                    " - {}{} = {}\n",
                    "$".yellow(),
                    Register::from_u32(rs).unwrap().to_lower_str().bold(),
                    runtime.state.read_register(rs).unwrap()
                ));

                error.push_str(&format!(
                    " - {}{} = {}\n",
                    "$".yellow(),
                    Register::from_u32(rt).unwrap().to_lower_str().bold(),
                    runtime.state.read_register(rt).unwrap()
                ));

                error
            }

            &Error::SegmentationFault { addr, access } => {
                let mut error = String::new();

                error.push_str("segmentation fault\n");

                match access {
                    SegmentationFaultAccessType::Read => {
                        error.push_str(&format!(
                            "\nthis happened because you tried to {} from\n",
                            "read".yellow()
                        ));
                        error.push_str(&format!(
                            "the address `{}{}`, which is not a valid address to read from\n",
                            "0x".bold(),
                            format!("{:08x}", addr).bold()
                        ));
                    }
                    SegmentationFaultAccessType::Write => {
                        error.push_str(&format!(
                            "\nthis happened because you tried to {} to\n",
                            "write".yellow()
                        ));
                        error.push_str(&format!(
                            "the address `{}{}`, which is not a valid address to write to\n",
                            "0x".bold(),
                            format!("{:08x}", addr).bold()
                        ));
                    }
                    SegmentationFaultAccessType::Execute => {
                        error.push_str(&format!(
                            "\nthis happened because you tried to {}\n",
                            "execute".yellow()
                        ));
                        error.push_str(&format!(
                            "the address `{}{}`, which is not a valid address to execute\n",
                            "0x".bold(),
                            format!("{:08x}", addr).bold()
                        ));
                    }
                }

                if access == SegmentationFaultAccessType::Read
                    || access == SegmentationFaultAccessType::Write
                {
                    let inst = runtime.state.read_mem_pc().unwrap();

                    if let ErrorContext::Binary | ErrorContext::Interactive = context {
                        error.push_str("\nthe instruction that failed was:\n");
                        error.push_str(&format!("{:#X}", inst));
                        error.push('\n');
                    }
                } else if runtime.get_segment(runtime.state.pc()) == Segment::Text
                {
                    let inst = runtime.state.read_mem_word(runtime.state.pc() - 4).unwrap();

                    if let ErrorContext::Binary | ErrorContext::Interactive = context {
                        error.push_str("\nthe instruction that got us here was:\n");
                        error.push_str(&format!("{:#X}", inst));
                        error.push('\n');
                    }
                }

                error.push('\n');

                error
            }

            &Error::InvalidSyscall { syscall, reason } => {
                let mut error = String::new();

                error.push_str("Invalid Syscall\n");

                match reason {
                    InvalidSyscallReason::Unimplemented => {
                        error.push_str(&format!(
                            "\nthe syscall number `{}` is not implemented.\n",
                            syscall.to_string().bold()
                        ));
                    }
                    InvalidSyscallReason::Unknown => {
                        error.push_str(&format!(
                            "\nthe syscall number `{}` is not valid.\n",
                            syscall.to_string().bold()
                        ));
                    }
                }

                let inst = runtime.state.read_mem_word(runtime.state.pc()).unwrap();

                if let ErrorContext::Binary | ErrorContext::Interactive = context {
                    error.push_str("\nthe instruction that failed was:\n");
                    error.push_str(&format!("{:#X}", inst));
                    error.push('\n');
                }

                error.push_str(&format!(
                    "\nthis happened because {}{} was `{}`.\n",
                    "$".yellow(),
                    "v0".white().bold(),
                    syscall.to_string().bold(),
                ));

                error.push('\n');

                error
            }
        }
    }
}