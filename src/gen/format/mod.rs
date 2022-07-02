use std::fmt::{self, Display};
use std::io::BufRead;
use std::str::FromStr;

use quick_xml::events::Event;
use quick_xml::Reader;
use tokio::fs::File;

use crate::error::ParseFormatError;
use crate::gen::Config;

mod plain;

const DEFAULT_LINE_WIDTH: usize = 80;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum Format {
    Plain {
        line_width: usize,
        linebreak: LinebreakStyle,
    },
    Markdown {
        line_width: usize,
    },
    Html,
}

impl Format {
    pub async fn write<R: BufRead>(
        &self,
        mut xml: Reader<R>,
        out: &mut File,
        config: &Config,
    ) -> anyhow::Result<()> {
        let mut buffer = Vec::new();

        loop {
            match xml.read_event(&mut buffer)? {
                Event::Start(e) if e.name() == b"text" => match self {
                    Self::Plain {
                        line_width,
                        linebreak,
                    } => plain::write(&mut xml, out, config, *linebreak, *line_width).await?,
                    Self::Markdown { .. } => todo!(),
                    Self::Html => todo!(),
                },
                Event::Eof => break,
                _ => {}
            }

            buffer.clear();
        }

        Ok(())
    }
}

impl Default for Format {
    fn default() -> Self {
        Self::Plain {
            line_width: DEFAULT_LINE_WIDTH,
            linebreak: LinebreakStyle::default(),
        }
    }
}

impl Display for Format {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Plain {
                line_width,
                linebreak,
            } => write!(f, "plain,{line_width},{linebreak}"),
            Self::Markdown { line_width } => write!(f, "markdown,{line_width}"),
            Self::Html => write!(f, "html"),
        }
    }
}

impl FromStr for Format {
    type Err = ParseFormatError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.split(',');
        let format = iter.next().unwrap(); // at least the format must be present, otherwise something went wrong
        match format {
            "plain" => Ok(Self::Plain {
                line_width: iter
                    .next()
                    .map(str::parse)
                    .transpose()?
                    .unwrap_or(DEFAULT_LINE_WIDTH),
                linebreak: iter.next().map(str::parse).transpose()?.unwrap_or_default(),
            }),
            "markdown" | "md" => Ok(Self::Markdown {
                line_width: iter
                    .next()
                    .map(str::parse)
                    .transpose()?
                    .unwrap_or(DEFAULT_LINE_WIDTH),
            }),
            "html" => Ok(Self::Html),
            _ => Err(ParseFormatError),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum LinebreakStyle {
    #[default]
    Unix,
    Windows,
}

impl LinebreakStyle {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Unix => "\n",
            Self::Windows => "\r\n",
        }
    }

    pub fn as_bytes(&self) -> &'static [u8] {
        self.as_str().as_bytes()
    }
}

impl Display for LinebreakStyle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Unix => write!(f, "unix"),
            Self::Windows => write!(f, "windows"),
        }
    }
}

impl FromStr for LinebreakStyle {
    type Err = ParseFormatError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "unix" => Ok(Self::Unix),
            "windows" => Ok(Self::Windows),
            _ => Err(ParseFormatError),
        }
    }
}
