use crate::game::Room;
use crate::Clients;
use crate::Messages;
use warp::ws::Message;

pub async fn main_loop(room: &Room, clients: &Clients, messages: &Messages) {
  // Handle messages
  let mut messages = messages.write().await;
  for (client_id, client_messages) in messages.clone().iter() {
    for client_message in client_messages {
      handle_message(client_id, client_message.clone(), clients, room).await;
    }
    messages.remove(client_id);
  }

  // Send game state
  for client in clients.read().await.values() {
    if let Some(sender) = &client.sender {
      let _ = sender.send(Ok(Message::text(room.to_datastring().await)));
    }
  }
}

async fn handle_message(id: &str, msg: Message, clients: &Clients, room: &Room) {
  let message = match std::str::from_utf8(msg.as_bytes()) {
    Ok(value) => value,
    Err(_) => return,
  };

  let message: Vec<&str> = message.split("/").collect();
  if message.len() != 2 {
    println!("Bad message");
    return;
  }

  let message_name = message[0];
  let message_payload = message[1];

  if let Some(client) = clients.read().await.get(id).cloned() {
    let mut locked = room.players.write().await;

    if let Some(player) = locked.get_mut(&client.player_id) {
      player.handle_message(message_name, message_payload)
    }
  }
}
