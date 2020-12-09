#[derive(Debug, Clone)]
pub struct Player {
  pub id: String,
  pub client_id: String,
  pub name: String,
  pub position: (isize, isize),
}

impl Player {
  pub fn handle_message(&mut self, name: &str, payload: &str) {
    if name != "move" {
      return;
    };

    if payload == "right" {
      self.position.0 += 10;
    } else if payload == "left" {
      self.position.0 -= 10;
    } else if payload == "down" {
      self.position.1 -= 10;
    } else if payload == "up" {
      self.position.1 += 10;
    }
  }

  pub fn to_datastring(&self) -> String {
    let (x, y) = self.position;
    format!("{};{};{}", self.id, x, y)
  }
}
