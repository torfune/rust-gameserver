use warp::ws::Message;

#[derive(Debug, Clone)]
pub struct Player {
  id: String,
  client_id: String,
  name: String,
  pub position: (isize, isize),
  last_input_sequence: isize,
}

impl Player {
  pub fn new(id: String, client_id: String) -> Self {
    Player {
      id,
      client_id,
      name: String::from("Test Player"),
      position: (0, 0),
      last_input_sequence: 0,
    }
  }

  pub fn handle_message(&mut self, message: &Message) {
    let message = match std::str::from_utf8(message.as_bytes()) {
      Ok(value) => value,
      Err(_) => return,
    };
    let message: Vec<&str> = message.split("/").collect();
    if message.len() != 2 {
      println!("Bad message");
      return;
    }
    let name = message[0];
    let payload = message[1];

    if name != "move" {
      return;
    };

    let payload: Vec<&str> = payload.split(";").collect();
    if payload.len() != 3 {
      println!("Bad payload");
      return;
    }

    let axis = payload[0];
    let value = payload[1];
    let input_sequence = payload[2];

    if let Ok(value) = value.parse::<isize>() {
      match axis {
        "x" => self.position.0 += value,
        "y" => self.position.1 += value,
        _ => {
          println!("Invalid axis: {}", axis);
          return;
        }
      }

      if let Ok(input_sequence) = input_sequence.parse::<isize>() {
        self.last_input_sequence = input_sequence;
      }
    }
  }

  pub fn to_datastring(&self) -> String {
    let (x, y) = self.position;
    format!("{};{};{};{}", self.id, x, y, self.last_input_sequence)
  }
}
