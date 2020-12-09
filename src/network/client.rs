use tokio::sync::mpsc;
use warp::ws::Message;

#[derive(Clone, Debug)]
pub struct Client {
  pub user_id: usize,
  pub player_id: String,
  pub sender: Option<mpsc::UnboundedSender<std::result::Result<Message, warp::Error>>>,
}
