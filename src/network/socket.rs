use crate::{Client, Clients, Room};
use futures::{FutureExt, StreamExt};
use tokio::sync::mpsc;
use warp::ws::{Message, WebSocket};

pub async fn connection(
  ws: WebSocket,
  id: String,
  clients: Clients,
  room: Room,
  mut client: Client,
) {
  let (client_ws_sender, mut client_ws_receiver) = ws.split();
  let (client_sender, client_receiver) = mpsc::unbounded_channel();

  tokio::task::spawn(client_receiver.forward(client_ws_sender).map(|result| {
    if let Err(e) = result {
      eprintln!("Error sending websocket message: {}", e);
    }
  }));

  client.sender = Some(client_sender);
  clients.write().await.insert(id.clone(), client.clone());

  println!("Client connected");

  // Send initial room state
  if let Some(sender) = &client.sender {
    let _ = sender.send(Ok(Message::text(room.to_datastring().await)));
  }

  // Handle messages until closed
  while let Some(result) = client_ws_receiver.next().await {
    let msg = match result {
      Ok(msg) => msg,
      Err(e) => {
        eprintln!("Error receiving websocket message: {}", e);
        break;
      }
    };

    handle_message(&id, msg, &clients, &room).await;
  }

  clients.write().await.remove(&id);
  println!("Client disconnected");
}

async fn handle_message(id: &str, msg: Message, clients: &Clients, room: &Room) {
  let message = match std::str::from_utf8(msg.as_bytes()) {
    Ok(value) => value,
    Err(_) => return,
  };

  let message: Vec<&str> = message.split("|").collect();
  if message.len() != 2 {
    println!("Bad message");
    return;
  }

  let message_name = message[0];
  let message_payload = message[1];

  println!("Message Name: {}", message_name);
  println!("Message Payload: {}", message_payload);

  if let Some(client) = clients.read().await.get(id).cloned() {
    let mut locked = room.players.write().await;

    if let Some(player) = locked.get_mut(&client.player_id) {
      player.handle_message(message_name, message_payload)
    }
  }
}
