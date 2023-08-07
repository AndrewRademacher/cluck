use std::collections::HashMap;

use anyhow::{anyhow, Result};
use tokio::sync::{mpsc, oneshot};
use uuid::Uuid;

use crate::watch::{Watch, WatchId};

#[derive(Clone)]
pub struct Group(mpsc::UnboundedSender<Message>);

struct GroupInner {
    rx: mpsc::UnboundedReceiver<Message>,
    watches: HashMap<WatchId, Watch>,
    waits: Vec<oneshot::Sender<()>>,
}

enum Message {
    AddWatch(Watch),
    ExitWatch(WatchId),
    Wait(oneshot::Sender<()>),
}

impl Group {
    pub fn new() -> Self {
        let (tx, rx) = mpsc::unbounded_channel();
        tokio::spawn(GroupInner::new(rx).run());
        Self(tx)
    }

    pub fn add_watch(&self, watch: Watch) -> Result<()> {
        match self.0.send(Message::AddWatch(watch)) {
            Ok(_) => Ok(()),
            Err(_) => Err(anyhow!("group is closed")),
        }
    }

    pub fn exit_watch(&self, watch_id: Uuid) {
        let _ = self.0.send(Message::ExitWatch(watch_id));
    }

    pub async fn wait(&self) {
        let (tx, rx) = oneshot::channel();
        match self.0.send(Message::Wait(tx)) {
            Ok(_) => {
                rx.await.unwrap();
            }
            Err(_) => {}
        }
    }
}

impl GroupInner {
    fn new(rx: mpsc::UnboundedReceiver<Message>) -> Self {
        Self {
            rx,
            watches: Default::default(),
            waits: Default::default(),
        }
    }

    async fn run(mut self) {
        match self.run_inner().await {
            Ok(_) => {}
            Err(e) => println!("[] {e:?}"),
        }
    }

    async fn run_inner(&mut self) -> Result<()> {
        loop {
            if let Some(msg) = self.rx.recv().await {
                let cont = match msg {
                    Message::AddWatch(watch) => self.add_watch(watch)?,
                    Message::ExitWatch(watch_id) => self.exit_watch(watch_id)?,
                    Message::Wait(wait) => self.add_wait(wait)?,
                };
                if !cont {
                    break;
                }
            } else {
                break;
            }
        }

        let waits = std::mem::replace(&mut self.waits, Default::default());
        waits.into_iter().for_each(|v| v.send(()).unwrap());
        Ok(())
    }

    fn add_watch(&mut self, watch: Watch) -> Result<bool> {
        self.watches.insert(watch.id(), watch);
        Ok(true)
    }

    fn exit_watch(&mut self, watch_id: WatchId) -> Result<bool> {
        self.watches.remove(&watch_id);
        if self.watches.is_empty() {
            Ok(false)
        } else {
            Ok(true)
        }
    }

    fn add_wait(&mut self, wait: oneshot::Sender<()>) -> Result<bool> {
        self.waits.push(wait);
        Ok(true)
    }
}
