use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
#[error("unrecognized license format")]
pub struct ParseFormatError;

#[derive(Debug, Error, PartialEq, Eq)]
#[error("unrecognized path")]
pub struct ParsePathError;
