use std::hash::Hash;

use anyhow::{anyhow, Result};
use tokio::{
    io::{AsyncBufReadExt, BufReader, Lines},
    process::{Child, ChildStderr, ChildStdout},
    sync::mpsc,
};
use uuid::Uuid;

use crate::group::Group;

#[derive(Clone)]
pub struct Watch(mpsc::UnboundedSender<Message>, WatchId);

pub type WatchId = Uuid;

struct WatchInner {
    id: Uuid,
    rx: mpsc::UnboundedReceiver<Message>,
    group: Group,
    child: Child,
    label: String,
    stdout: Lines<BufReader<ChildStdout>>,
    stderr: Lines<BufReader<ChildStderr>>,
}

enum Message {}

impl Watch {
    pub fn new(group: Group, child: Child, label: String) -> Result<Self> {
        let (tx, rx) = mpsc::unbounded_channel();
        let id = Uuid::new_v4();
        tokio::spawn(WatchInner::new(id, rx, group, child, label)?.run());
        Ok(Self(tx, id))
    }

    pub fn id(&self) -> WatchId {
        self.1
    }
}

impl Hash for Watch {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.1.hash(state)
    }
}

impl WatchInner {
    fn new(
        id: WatchId,
        rx: mpsc::UnboundedReceiver<Message>,
        group: Group,
        mut child: Child,
        label: String,
    ) -> Result<Self> {
        let stdout = child
            .stdout
            .take()
            .ok_or_else(|| anyhow!("child did not have stdout"))?;
        let stderr = child
            .stderr
            .take()
            .ok_or_else(|| anyhow!("child did not have stderr"))?;
        Ok(Self {
            id,
            rx,
            group,
            child,
            label,
            stdout: BufReader::new(stdout).lines(),
            stderr: BufReader::new(stderr).lines(),
        })
    }

    async fn run(mut self) {
        match self.run_inner().await {
            Ok(_) => {}
            Err(e) => println!("[] {e:?}"),
        }
    }

    async fn run_inner(&mut self) -> Result<()> {
        loop {
            tokio::select! {
                Some(msg) = self.rx.recv() => {
                    self.handle_message(msg).await?;
                },
                res = self.stdout.next_line() => {
                    if !self.handle_stdout(res).await? {
                        break;
                    }
                },
                res = self.stderr.next_line() => {
                    if !self.handle_stderr(res).await? {
                        break;
                    }
                }
                else => break,
            }
        }

        self.group.exit_watch(self.id);
        Ok(())
    }

    async fn handle_message(&mut self, res: Message) -> Result<()> {
        match res {}
    }

    async fn handle_stdout(&mut self, res: std::io::Result<Option<String>>) -> Result<bool> {
        let res = res?;
        match res {
            Some(value) => {
                println!("[{}] {}", &self.label, &value);
                Ok(true)
            }
            None => Ok(false),
        }
    }

    async fn handle_stderr(&mut self, res: std::io::Result<Option<String>>) -> Result<bool> {
        let res = res?;
        match res {
            Some(value) => {
                println!("[{}] {}", &self.label, &value);
                Ok(true)
            }
            None => Ok(false),
        }
    }
}
