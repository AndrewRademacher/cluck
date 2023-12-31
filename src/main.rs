use anyhow::Result;
use clap::Parser;
use cluckfile::Cluckfile;
use console::Style;
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

    let file = if args.stdin {
        Cluckfile::from_stdin().await?
    } else if let Some(path) = args.cluckfile {
        Cluckfile::from_file(path).await?
    } else {
        Cluckfile::from_environment().await?
    };

    run(file).await
}

pub enum RootMessage {}

async fn run(args: Cluckfile) -> Result<()> {
    let group = Group::new();

    for (color, (label, command)) in args.cmd.into_iter().enumerate() {
        let child = command.boot()?;
        let watch = Watch::new(
            group.clone(),
            child,
            label,
            Style::new().color256(color as u8 + 1),
        )?;
        group.add_watch(watch)?;
    }

    group.wait().await;
    Ok(())
}
