use crate::room::Room;
use parking_lot::RwLock;
use std::collections::HashMap;
use std::sync::Arc;

pub struct RoomManager<T : Clone, Message: Clone> {
    rooms: RwLock<HashMap<u64, Arc<Room<T, Message>>>>,
}

impl<T : Clone, Message: Clone> RoomManager<T, Message> {
    pub fn new() -> Self {
        Self {
            rooms: RwLock::new(HashMap::new()),
        }
    }

    pub fn send<F>(&self, room_id: u64, message_factory: F) -> anyhow::Result<()>
    where
      F: FnOnce(u64) -> anyhow::Result<Message>,
    {
        let read = self.rooms.read();

        let room = read
            .get(&room_id)
            .ok_or_else(|| anyhow::anyhow!("Room {} not found", room_id))?;

        let message = message_factory(room.next_event_id())?;

        room.broadcast(message)
    }

    pub fn get(&self, room_id: u64) -> anyhow::Result<Arc<Room<T, Message>>> {
        let read = self.rooms.read();

        Ok(
            read
              .get(&room_id)
              .ok_or_else(|| anyhow::anyhow!("Room {} not found", room_id))?
              .clone()
        )
    }

    pub fn get_or_create(&self, room_id: u64) -> Arc<Room<T, Message>> {
        let read = self.rooms.upgradable_read();

        if let Some(r) = read.get(&room_id) {
            return r.clone();
        }

        let room = Arc::new(Room::new());
        
        tracing::info!("Room {} created", room_id);

        let mut write = parking_lot::RwLockUpgradableReadGuard::upgrade(read);
        write.insert(room_id, room.clone());

        room
    }
}
