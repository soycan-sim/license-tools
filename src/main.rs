use clap::{Parser, Subcommand};

use git2::Config;
use time::OffsetDateTime;

use license_tools::fmt::Format;
use license_tools::paths::Paths;

#[derive(Debug, Parser)]
#[clap(author, version, about)]
struct Cli {
    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Generate license file(s).
    Gen {
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
    },
}

fn main() {
    let cli = Cli::parse();

    dbg!(cli);
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
    let config = Config::open_default().ok()?;
    config.get_string("user.name").ok()
}
