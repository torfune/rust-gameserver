use crate::utils;
use warp::ws::Message;

const MOVEMENT_SPEED: isize = 50;
const GRAVITY_FORCE: isize = 2;
const FLOOR_Y: isize = 400;
const JUMP_FORCE: isize = 25;
const PLAYER_HEIGHT: isize = 100;

#[derive(Debug, Clone)]
pub struct Player {
  id: String,
  name: String,
  last_input_sequence: isize,
  velocity_y: isize,
  pub client_id: String,
  pub position: (isize, isize),
}

impl Player {
  pub fn new(id: String, client_id: String) -> Self {
    Player {
      id,
      client_id,
      name: String::from("Test Player"),
      position: (0, 0),
      last_input_sequence: 0,
      velocity_y: 0,
    }
  }

  pub fn update(&mut self, messages: Option<&Vec<Message>>) {
    // Handle user input
    match messages {
      Some(messages) => {
        let old_position_x = self.position.0;
        for client_message in messages.iter() {
          self.handle_message(client_message);
        }

        // Correct position
        let new_position_x = self.position.0;
        let delta_x = new_position_x - old_position_x;
        if delta_x.abs() > MOVEMENT_SPEED {
          if delta_x > 0 {
            self.position.0 = old_position_x + MOVEMENT_SPEED;
          } else {
            self.position.0 = old_position_x - MOVEMENT_SPEED;
          }
        }
      }
      None => (),
    }

    // Gravity
    self.velocity_y += GRAVITY_FORCE;
    self.position.1 += self.velocity_y;
    if self.position.1 + PLAYER_HEIGHT > FLOOR_Y {
      self.velocity_y = 0;
      self.position.1 = FLOOR_Y - PLAYER_HEIGHT;
    }
  }

  pub fn handle_message(&mut self, message: &Message) {
    let (name, payload) = match utils::split_message(message) {
      Ok(split_message) => split_message,
      Err(_) => return,
    };

    match name {
      "move" => {
        let payload: Vec<&str> = payload.split(";").collect();
        if payload.len() != 3 {
          return;
        };

        let axis = payload[0];
        let value = payload[1];
        let input_sequence = payload[2];

        if let Ok(value) = value.parse::<isize>() {
          match axis {
            "x" => self.position.0 += value,
            "y" => self.position.1 += value,
            _ => return,
          }
          if let Ok(input_sequence) = input_sequence.parse::<isize>() {
            self.last_input_sequence = input_sequence;
          }
        }
      }
      "jump" => {
        self.velocity_y = -JUMP_FORCE;
      }
      _ => (),
    }
  }

  pub fn to_datastring(&self) -> String {
    let (x, y) = self.position;
    format!("{};{};{};{}", self.id, x, y, self.last_input_sequence)
  }
}

// -... / .- / ..- / ---- // -- / --- / -. / . / -.--
