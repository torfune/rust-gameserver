use crate::game::Room;
use crate::network::Client;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use warp::ws::Message;

pub async fn main_loop(room: &Room, clients: &Arc<RwLock<HashMap<String, Client>>>) {
  for client in clients.read().await.values() {
    if let Some(sender) = &client.sender {
      let _ = sender.send(Ok(Message::text(room.to_datastring().await)));
    }
  }
}
