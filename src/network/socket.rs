use crate::{Client, Clients, Messages};
use futures::{FutureExt, StreamExt};
use tokio::sync::mpsc;
use warp::ws::{Message, WebSocket};

pub async fn connection(
  ws: WebSocket,
  client_id: String,
  clients: Clients,
  messages: Messages,
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
  clients
    .write()
    .await
    .insert(client_id.clone(), client.clone());

  println!("Client connected");

  // Send initial data
  if let Some(sender) = &client.sender {
    let _ = sender.send(Ok(Message::text(format!("id/{}", client.player_id))));
  }

  while let Some(result) = client_ws_receiver.next().await {
    let message = match result {
      Ok(message) => message,
      Err(e) => {
        eprintln!("Error receiving websocket message: {}", e);
        break;
      }
    };

    let mut client_messages = match messages.write().await.get_mut(&client_id) {
      Some(client_messages) => client_messages.clone(),
      None => Vec::new(),
    };

    client_messages.push(message);
    messages
      .write()
      .await
      .insert(client_id.clone(), client_messages);
  }

  clients.write().await.remove(&client_id);
  println!("Client disconnected");
}
