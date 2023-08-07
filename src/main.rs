use anyhow::Result;
use clap::Parser;
use cluckfile::Cluckfile;
use group::Group;
use watch::Watch;

use crate::args::Args;

mod args;
mod cluckfile;
mod group;
mod watch;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    let args = Args::parse();
    match args.command {
        Some(cmd) => match cmd {
            args::Command::Run(args) => run(Cluckfile::from(args)).await,
        },
        None => run(Cluckfile::from_environment().await?).await,
    }
}

pub enum RootMessage {}

async fn run(args: Cluckfile) -> Result<()> {
    let group = Group::new();

    for command in args.commands {
        let (label, child) = command.boot()?;
        let watch = Watch::new(group.clone(), child, label)?;
        group.add_watch(watch)?;
    }

    group.wait().await;
    Ok(())
}
