use crate::game::Room;
use crate::Clients;
use crate::Messages;
use warp::ws::Message;

pub async fn main_loop(room: &Room, clients: &Clients, messages: &Messages) {
  // Handle input and update game state
  let mut messages = messages.write().await;

  // Update game state
  {
    let mut players = room.players.write().await;
    for (_, player) in players.iter_mut() {
      let client_messages = messages.get(&player.client_id);
      player.update(client_messages);
      messages.remove(&player.client_id);
    }
  }

  // Send game state
  for client in clients.read().await.values() {
    if let Some(sender) = &client.sender {
      let _ = sender.send(Ok(Message::text(room.to_datastring().await)));
    }
  }
}
