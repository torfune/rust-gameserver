#[derive(Debug, Clone)]
pub struct Player {
  id: String,
  client_id: String,
  name: String,
  position: (isize, isize),
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

  pub fn handle_message(&mut self, name: &str, payload: &str) {
    if name != "move" {
      return;
    };

    let payload: Vec<&str> = payload.split("|").collect();
    if payload.len() != 2 {
      println!("Bad payload");
      return;
    }

    let direction = payload[0];
    let input_sequence = payload[1];

    match direction {
      "right" => self.position.0 += 10,
      "left" => self.position.0 -= 10,
      "down" => self.position.1 += 10,
      "up" => self.position.1 -= 10,
      _ => {
        println!("Bad direction: {}", direction);
        return;
      }
    }

    if let Ok(input_sequence) = input_sequence.parse::<isize>() {
      self.last_input_sequence = input_sequence;
    }
  }

  pub fn to_datastring(&self) -> String {
    let (x, y) = self.position;
    format!("{};{};{};{}", self.id, x, y, self.last_input_sequence)
  }
}
