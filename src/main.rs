use clap::{Parser, Subcommand};

pub mod error;
pub mod gen;
pub mod paths;

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
        #[clap(flatten)]
        config: gen::Config,
    },
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Gen { config }) => gen::generate(config).await,
        None => Ok(()),
    }
}
