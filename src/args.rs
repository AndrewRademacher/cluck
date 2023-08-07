use std::path::PathBuf;

use clap::Parser;

#[derive(Debug, Parser)]
pub struct Args {
    /// Use a specific cluck configuration file.
    #[clap(short = 'c', long = "cluckfile")]
    pub cluckfile: Option<PathBuf>,
    #[clap(subcommand)]
    pub command: Option<Command>,
}

#[derive(Debug, Parser)]
pub enum Command {
    /// Run shell commands directly and together.
    Run(Run),
}

#[derive(Debug, Parser)]
pub struct Run {
    /// Shell commands that will be run together.
    pub commands: Vec<String>,
}
