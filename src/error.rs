use std::num::ParseIntError;

use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
#[error("unrecognized license format")]
pub struct ParseFormatError;

impl From<ParseIntError> for ParseFormatError {
    fn from(_: ParseIntError) -> Self {
        Self
    }
}

#[derive(Debug, Error, PartialEq, Eq)]
#[error("unrecognized path")]
pub struct ParsePathError;
