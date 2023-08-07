use std::{
    collections::HashMap,
    path::{Path, PathBuf},
    process::Stdio,
    str::FromStr,
};

use anyhow::{anyhow, Result};
use serde::Deserialize;
use tokio::{io::AsyncReadExt, process::Child};

#[derive(Debug, Deserialize)]
pub struct Cluckfile {
    pub cmd: HashMap<Name, Command>,
}

pub type Name = String;

#[derive(Debug, Deserialize)]
pub struct Command {
    pub exec: String,
    #[serde(default)]
    pub environment: HashMap<String, String>,
}

impl Command {
    pub fn boot(self) -> Result<Child> {
        let mut parts_iter = self.exec.split(' ');
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

        Ok(prog.spawn()?)
    }
}

impl FromStr for Cluckfile {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Ok(toml::from_str(s)?)
    }
}

impl Cluckfile {
    pub async fn from_file(path: impl AsRef<Path>) -> Result<Self> {
        Ok(toml::from_str(&tokio::fs::read_to_string(path).await?)?)
    }

    pub async fn from_environment() -> Result<Self> {
        Ok(toml::from_str(
            &tokio::fs::read_to_string(Self::find_file()?).await?,
        )?)
    }

    pub async fn from_stdin() -> Result<Self> {
        let mut file = Vec::new();
        let mut stdin = tokio::io::stdin();
        stdin.read_to_end(&mut file).await?;
        let str = std::str::from_utf8(&file)?;
        Ok(toml::from_str(str)?)
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
