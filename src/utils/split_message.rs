use warp::ws::Message;

pub struct SplitMessageError;

pub fn split_message(message: &Message) -> Result<(&str, &str), SplitMessageError> {
  let message = match std::str::from_utf8(message.as_bytes()) {
    Ok(value) => value,
    Err(_) => return Err(SplitMessageError),
  };

  let message: Vec<&str> = message.split("/").collect();
  if message.len() != 2 {
    println!("Bad message");
    return Err(SplitMessageError);
  }

  let name = message[0];
  let payload = message[1];
  return Ok((name, payload));
}
