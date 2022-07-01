use std::fmt::{self, Display};
use std::str::FromStr;

use crate::error::ParseFormatError;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[non_exhaustive]
pub enum Format {
    #[default]
    Plain,
    Markdown,
    Html,
}

impl Display for Format {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Plain => write!(f, "plain"),
            Self::Markdown => write!(f, "markdown"),
            Self::Html => write!(f, "html"),
        }
    }
}

impl FromStr for Format {
    type Err = ParseFormatError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "plain" => Ok(Self::Plain),
            "markdown" | "md" => Ok(Self::Markdown),
            "html" => Ok(Self::Html),
            _ => Err(ParseFormatError),
        }
    }
}
