use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};
use log::Level::Trace;
use parking_lot::RwLock;
use tokio::sync::{broadcast};
use log::trace;

pub struct Room<T : Clone, Message : Clone> {
    members: RwLock<HashMap<u64, T>>,
    broadcast_channel: broadcast::Sender<Message>,
    event_id: AtomicU64,
}

impl <T : Clone, Message: Clone> Room<T, Message> {
    pub(crate) fn new() -> Self {
        let (tx, _) = broadcast::channel(128);

        Self {
            members: RwLock::new(HashMap::new()),
            broadcast_channel: tx,
            event_id: AtomicU64::new(0),
        }
    }

    pub fn get_members(&self) -> Vec<T> {
        self.members.read().values().cloned().collect()
    }

    pub fn next_event_id(&self) -> u64 {
        self.event_id.fetch_add(1, Ordering::Relaxed)
    }

    pub fn join(&self, session_id: u64, member: T) -> bool {
        let read = self.members.upgradable_read();

        if read.contains_key(&session_id) {
            return false;
        }

        let mut write = parking_lot::RwLockUpgradableReadGuard::upgrade(read);
        write.insert(session_id, member);

        true
    }

    pub fn broadcast(&self, message: Message) -> anyhow::Result<()> {
        self.broadcast_channel
          .send(message)
          .map_err(|e| {
              tracing::error!("Failed to fire join event : {e}");
              anyhow::anyhow!("broadcast failed")
          })?;

        Ok(())
    }

    pub fn subscribe(&self) -> broadcast::Receiver<Message> {
        self.broadcast_channel.subscribe()
    }
}