use std::io::BufRead;

use quick_xml::events::Event;
use quick_xml::Reader;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;

use crate::gen::format::LinebreakStyle;
use crate::gen::Config;

pub async fn write<R: BufRead>(
    xml: &mut Reader<R>,
    out: &mut File,
    config: &Config,
    linebreak: LinebreakStyle,
    max_line_width: usize,
) -> anyhow::Result<()> {
    let mut buffer = Vec::new();

    loop {
        match xml.read_event(&mut buffer)? {
            Event::Start(e) => match e.name() {
                b"titleText" => write_title_text(xml, out, config, linebreak).await?,
                b"copyrightText" if !config.no_copyright => {
                    write_copyright_text(xml, out, config, linebreak).await?
                }
                b"p" => write_p(xml, out, config, linebreak, max_line_width).await?,
                _ => {}
            },
            Event::End(e) if e.name() == b"text" => break,
            _ => {}
        }

        buffer.clear();
    }

    Ok(())
}

pub async fn write_title_text<R: BufRead>(
    xml: &mut Reader<R>,
    out: &mut File,
    _config: &Config,
    linebreak: LinebreakStyle,
) -> anyhow::Result<()> {
    let mut buffer = Vec::new();

    loop {
        match xml.read_event(&mut buffer)? {
            Event::Text(e) => {
                out.write_all(e.unescape_and_decode(xml)?.as_bytes())
                    .await?
            }
            Event::End(e) => match e.name() {
                b"p" => {
                    out.write_all(linebreak.as_bytes()).await?;
                    out.write_all(linebreak.as_bytes()).await?;
                }
                b"titleText" => break,
                _ => {}
            },
            _ => {}
        }

        buffer.clear();
    }

    Ok(())
}

pub async fn write_copyright_text<R: BufRead>(
    xml: &mut Reader<R>,
    out: &mut File,
    _config: &Config,
    linebreak: LinebreakStyle,
) -> anyhow::Result<()> {
    let mut buffer = Vec::new();

    loop {
        match xml.read_event(&mut buffer)? {
            Event::Text(e) => {
                // TODO: replace year and copyright holder
                let copyright = e.unescape_and_decode(xml)?;
                out.write_all(copyright.as_bytes()).await?;
            }
            Event::End(e) => match e.name() {
                b"p" => {
                    out.write_all(linebreak.as_bytes()).await?;
                    out.write_all(linebreak.as_bytes()).await?;
                }
                b"copyrightText" => break,
                _ => {}
            },
            _ => {}
        }

        buffer.clear();
    }

    Ok(())
}

pub async fn write_p<R: BufRead>(
    xml: &mut Reader<R>,
    out: &mut File,
    _config: &Config,
    linebreak: LinebreakStyle,
    max_line_width: usize,
) -> anyhow::Result<()> {
    let mut buffer = Vec::new();

    let mut line_width = 0;
    let mut space = false;

    loop {
        match xml.read_event(&mut buffer)? {
            Event::Text(e) => {
                let text = e.unescape_and_decode(xml)?;
                for word in text.split_whitespace() {
                    let is_too_long = line_width >= max_line_width;
                    let will_be_too_long = line_width + word.len() > max_line_width;
                    let word_too_long = word.len() > max_line_width;

                    if is_too_long || will_be_too_long && !word_too_long {
                        out.write_all(linebreak.as_bytes()).await?;
                        line_width = 0;
                        space = false;
                    }

                    if space {
                        out.write_all(b" ").await?;
                    }

                    out.write_all(word.as_bytes()).await?;
                    line_width += word.len();
                    space = true;
                }
            }
            Event::End(e) if e.name() == b"p" => {
                out.write_all(linebreak.as_bytes()).await?;
                out.write_all(linebreak.as_bytes()).await?;
                break;
            }
            _ => {}
        }

        buffer.clear();
    }

    Ok(())
}
