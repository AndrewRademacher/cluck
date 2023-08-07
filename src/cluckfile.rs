use std::{collections::HashMap, path::PathBuf, process::Stdio};

use anyhow::{anyhow, Result};
use serde::Deserialize;
use tokio::process::Child;

use crate::args::Run;

#[derive(Debug, Deserialize)]
pub struct Cluckfile {
    pub commands: Vec<Command>,
}

#[derive(Debug, Deserialize)]
pub struct Command {
    pub label: String,
    pub shell: String,
    pub environment: HashMap<String, String>,
}

impl Command {
    pub fn boot(self) -> Result<(String, Child)> {
        let mut parts_iter = self.shell.split(' ');
        let mut prog = tokio::process::Command::new(
            parts_iter
                .next()
                .ok_or_else(|| anyhow!("no program in command"))?,
        );
        for arg in parts_iter {
            prog.arg(arg);
        }
        for (k, v) in self.environment {
            prog.env(k, v);
        }

        prog.stdout(Stdio::piped());
        prog.stderr(Stdio::piped());

        Ok((self.label, prog.spawn()?))
    }
}

impl From<String> for Command {
    fn from(value: String) -> Self {
        Command {
            label: value.clone(),
            shell: value,
            environment: Default::default(),
        }
    }
}

impl From<Run> for Cluckfile {
    fn from(value: Run) -> Self {
        Self {
            commands: value.commands.into_iter().map(Command::from).collect(),
        }
    }
}

impl Cluckfile {
    pub async fn from_environment() -> Result<Self> {
        Ok(toml::from_str(
            &tokio::fs::read_to_string(Self::find_file()?).await?,
        )?)
    }

    fn find_file() -> Result<PathBuf> {
        let pwd = std::env::current_dir()?;

        let test = pwd.join("Cluck.toml");
        if test.exists() {
            return Ok(test);
        }

        let test = pwd.join("cluck.toml");
        if test.exists() {
            return Ok(test);
        }

        Err(anyhow!("no cluck configuration file found"))
    }
}
