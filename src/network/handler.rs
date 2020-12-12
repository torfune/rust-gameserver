use super::socket;
use super::Client;
use crate::Player;
use crate::{Clients, Messages, Result, Room};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use warp::{http::StatusCode, reply::json, Reply};

#[derive(Serialize, Debug)]
pub struct RegisterResponse {
  url: String,
}

#[derive(Deserialize, Debug)]
pub struct RegisterRequest {
  pub user_id: usize,
}

pub async fn handle_register(
  body: RegisterRequest,
  clients: Clients,
  room: Room,
) -> Result<impl Reply> {
  let user_id = body.user_id;
  let client_id = Uuid::new_v4().simple().to_string();
  let player_id = Uuid::new_v4().simple().to_string();

  let client = Client {
    user_id,
    player_id: player_id.clone(),
    sender: None,
  };

  clients.write().await.insert(client_id.clone(), client);
  room
    .players
    .write()
    .await
    .insert(player_id.clone(), Player::new(player_id, client_id.clone()));

  return Ok(json(&RegisterResponse {
    url: format!("ws://127.0.0.1:8000/ws/{}", client_id),
  }));
}

pub async fn handle_unregister(id: String, clients: Clients) -> Result<impl Reply> {
  clients.write().await.remove(&id);
  return Ok(StatusCode::OK);
}

pub async fn handle_websocket(
  ws: warp::ws::Ws,
  id: String,
  clients: Clients,
  messages: Messages,
) -> Result<impl Reply> {
  let client = clients.read().await.get(&id).cloned();
  return match client {
    Some(c) => {
      Ok(ws.on_upgrade(move |socket| socket::connection(socket, id, clients, messages, c)))
    }
    None => Err(warp::reject::not_found()),
  };
}
