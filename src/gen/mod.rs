use clap::Args;
use quick_xml::Reader;
use spdx::{Expression, ParseMode};
use time::OffsetDateTime;
use tokio::fs::File;

use self::format::Format;

use crate::paths::{Paths, DEFAULT_LICENSE};

mod format;

const GITHUB_SPDX: &str = "spdx";
const REPO_LICENSE_LIST: &str = "license-list-XML";

#[derive(Debug, Clone, Args)]
pub struct Config {
    /// Set copyright year. Defaults to current year.
    #[clap(long, parse(try_from_str), default_value_t = current_year())]
    year: u32,

    /// Set copyright holder. Defaults to 'user.name' from git config.
    #[clap(long, value_parser, default_value_t = default_copyright())]
    copyright: String,

    /// Don't add a copyright notice.
    #[clap(long, value_parser)]
    no_copyright: bool,

    /// Replace unicode characters with ascii.
    #[clap(long, value_parser)]
    ascii: bool,

    /// Format to output. Possible options: 'plain', 'markdown', 'html'.
    #[clap(long, parse(try_from_str), default_value_t)]
    format: Format,

    /// Path or comma-separated list of paths to output to.
    #[clap(long, parse(try_from_os_str = Paths::try_from), default_value_t)]
    out: Paths,

    /// SPDX license identifier or expression.
    #[clap(value_parser)]
    license: String,
}

pub async fn generate(config: Config) -> anyhow::Result<()> {
    let instance = octocrab::instance();
    let repo = instance.repos(GITHUB_SPDX, REPO_LICENSE_LIST);

    let license = Expression::parse_mode(&config.license, ParseMode::LAX)?;
    let version = format!("v{}", spdx::license_version());

    let license_count = license
        .requirements()
        .filter_map(|req| req.req.license.id())
        .count();

    let mut config = config;

    if config.out.len() != license_count {
        if config.out.len() == 1 {
            // unwrap is fine because we checked that len is 1
            let base = config.out.take().unwrap();
            let dir = base.parent().unwrap_or_else(|| ".".as_ref());
            let filename = base.file_stem().unwrap_or_else(|| DEFAULT_LICENSE.as_ref());
            let ext = base.extension();

            for id in license
                .requirements()
                .filter_map(|req| req.req.license.id())
            {
                let name = id.name;

                // spdx license ids use kebab case, we're gonna use this to generate unique-ish file names
                let uniq = name
                    .split_once('-')
                    .map(|(name, _)| name)
                    .unwrap_or(name)
                    .to_uppercase();

                let mut path = dir.to_path_buf();
                let mut filename = filename.to_os_string();
                filename.push("-");
                filename.push(&uniq);
                path.push(filename);
                if let Some(ext) = ext {
                    path.set_extension(ext);
                }

                config.out.push(path);
            }
        } else {
            anyhow::bail!("when generating multiple licenses, the number of OUT-paths must be either 1 or match the number of licenses");
        }
    }

    let config = config;

    let mut buffer = Vec::new();

    for (id, out) in license
        .requirements()
        .filter_map(|req| req.req.license.id())
        .zip(config.out.paths())
    {
        let name = id.name;
        let content = repo
            .get_content()
            .path(format!("src/{name}.xml"))
            .r#ref(&version)
            .send()
            .await?;
        if let Some(content) = &content.items[0].content {
            buffer.clear();

            for line in content.lines() {
                base64::decode_config_buf(line, base64::STANDARD, &mut buffer)?;
            }

            let mut xml = Reader::from_bytes(&buffer);
            xml.trim_text(true);

            let mut file = File::create(out).await?;
            config.format.write(xml, &mut file, &config).await?;
        } else {
            anyhow::bail!("no contents found in license data '{name}'");
        }
    }

    Ok(())
}

fn current_year() -> u32 {
    let datetime = OffsetDateTime::now_local().unwrap_or_else(|_| OffsetDateTime::now_utc());
    datetime.year() as _
}

fn default_copyright() -> String {
    const DEFAULT_COPYRIGHT: &str = "[copyright holder(s)]";
    git_user_name().unwrap_or_else(|| DEFAULT_COPYRIGHT.to_string())
}

fn git_user_name() -> Option<String> {
    let config = git2::Config::open_default().ok()?;
    config.get_string("user.name").ok()
}
