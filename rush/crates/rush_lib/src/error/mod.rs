use serde::{Deserialize, Serialize};
use std::rc::Rc;

pub mod compiler;
pub mod runtime;

pub type RushResult<T> = Result<T, RushError>;
pub type RuntimeError = runtime::RuntimeError;

pub type RushInternalResult<T> = Result<T, InternalError>;

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub enum RushError {
    Runtime(runtime::RuntimeError),
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub enum InternalError {
    Runtime(runtime::Error)
}

pub trait ToRushResult<T> {
    fn into_parser_rush_result(self, file_tag: Rc<str>, line: u32, col: u32) -> RushResult<T>;
    fn into_compiler_rush_result(
        self,
        file_tag: Rc<str>,
        line: u32,
        col: u32,
        col_end: u32,
    ) -> RushResult<T>;
    fn into_runtime_rush_result(self) -> RushResult<T>;
}

impl<T> ToRushResult<T> for RushInternalResult<T> {
    fn into_parser_rush_result(self, file_tag: Rc<str>, line: u32, col: u32) -> RushResult<T> {
        match self {
            Ok(t) => Ok(t),
            Err(error) => Err(error.into_parser_rush_error(file_tag, line, col)),
        }
    }

    fn into_compiler_rush_result(
        self,
        file_tag: Rc<str>,
        line: u32,
        col: u32,
        col_end: u32,
    ) -> RushResult<T> {
        match self {
            Ok(t) => Ok(t),
            Err(error) => Err(error.into_compiler_rush_error(file_tag, line, col, col_end)),
        }
    }

    fn into_runtime_rush_result(self) -> RushResult<T> {
        match self {
            Ok(t) => Ok(t),
            Err(error) => Err(error.into_runtime_rush_error()),
        }
    }
}

impl InternalError {
    pub fn into_parser_rush_error(self, _file_tag: Rc<str>, _line: u32, _col: u32) -> RushError {
        match self {
            InternalError::Runtime(error) => RushError::Runtime(RuntimeError::new(error)),
        }
    }

    pub fn into_compiler_rush_error(
        self,
        _file_tag: Rc<str>,
        _line: u32,
        _col: u32,
        _col_end: u32,
    ) -> RushError {
        match self {
            InternalError::Runtime(error) => RushError::Runtime(RuntimeError::new(error)),
        }
    }

    pub fn into_runtime_rush_error(self) -> RushError {
        match self {
            InternalError::Runtime(error) => RushError::Runtime(RuntimeError::new(error)),
        }
    }
}

#[macro_export]
macro_rules! cerr {
    ($err:expr) => {
        Err($crate::error::RushError::Compile($err))
    };
}

#[macro_export]
macro_rules! clerr {
    ($line:expr, $err:expr) => {
        Err($crate::error::RushError::CompileLine {
            line: $line,
            error: $err,
        })
    };
}

#[macro_export]
macro_rules! rerr {
    ($err:expr) => {
        Err($crate::error::RushError::Runtime($err))
    };
}
