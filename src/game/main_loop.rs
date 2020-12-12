use crate::game::Room;
use crate::Clients;
use crate::Messages;
use warp::ws::Message;

const SPEED: isize = 30;

pub async fn main_loop(room: &Room, clients: &Clients, messages: &Messages) {
  let mut messages = messages.write().await;
  for (client_id, client_messages) in messages.clone().iter() {
    if let Some(client) = clients.read().await.get(client_id) {
      if let Some(player) = room.players.write().await.get_mut(&client.player_id) {
        let old_position_x = player.position.0;
        for client_message in client_messages {
          player.handle_message(client_message);
        }

        // Lag position correction
        let new_position_x = player.position.0;
        let delta_x = new_position_x - old_position_x;
        if delta_x.abs() > SPEED {
          if delta_x > 0 {
            player.position.0 = old_position_x + SPEED;
          } else {
            player.position.0 = old_position_x - SPEED;
          }
        }
      }
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
