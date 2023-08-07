use std::path::PathBuf;

use clap::Parser;

#[derive(Debug, Parser)]
pub struct Args {
    /// Use a specific cluck configuration file.
    #[clap(short = 'c', long = "cluckfile")]
    pub cluckfile: Option<PathBuf>,
    /// Use a configuration file provided via stdin.
    #[clap(short = 's', long = "stdin")]
    pub stdin: bool,
}
