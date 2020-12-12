use super::player::Player;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Debug, Clone)]
pub struct Room {
  pub players: Arc<RwLock<HashMap<String, Player>>>,
}

impl Room {
  pub fn new() -> Self {
    let room = Room {
      players: Arc::new(RwLock::new(HashMap::new())),
    };

    return room;
  }

  pub async fn to_datastring(&self) -> String {
    let player_datastrings: Vec<String> = self
      .players
      .read()
      .await
      .iter()
      .map(|(_, player)| {
        let mut d = "".to_string();
        d.push_str(&player.to_datastring().to_string());
        d.push_str("|");
        return d;
      })
      .collect();

    let mut datastring: String = "players/".to_string();

    player_datastrings
      .iter()
      .for_each(|d| datastring.push_str(d));
    datastring.pop();

    return datastring;
  }
}
