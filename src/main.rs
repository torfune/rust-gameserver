use game::Player;
use game::Room;
use gameloop::{FrameAction, GameLoop};
use network::Client;
use std::collections::HashMap;
use std::convert::Infallible;
use std::sync::Arc;
use std::thread;
use tokio::runtime;
use tokio::sync::RwLock;
use warp::{Filter, Rejection};

mod game;
mod network;

type Result<T> = std::result::Result<T, Rejection>;
type Clients = Arc<RwLock<HashMap<String, Client>>>;

#[tokio::main]
async fn main() {
  let room = Room::new();
  let clients: Clients = Arc::new(RwLock::new(HashMap::new()));

  let register = warp::path("register");
  let register_routes = register
    .and(warp::post())
    .and(warp::body::json())
    .and(with_clients(clients.clone()))
    .and(with_room(room.clone()))
    .and_then(network::handle_register)
    .or(
      register
        .and(warp::delete())
        .and(warp::path::param())
        .and(with_clients(clients.clone()))
        .and_then(network::handle_unregister),
    );

  let ws_route = warp::path("ws")
    .and(warp::ws())
    .and(warp::path::param())
    .and(with_clients(clients.clone()))
    .and(with_room(room.clone()))
    .and_then(network::handle_websocket);

  let routes = register_routes.or(ws_route).with(
    warp::cors()
      .allow_any_origin()
      .allow_any_origin()
      .allow_headers(vec![
        "User-Agent",
        "Sec-Fetch-Mode",
        "Referer",
        "Origin",
        "Access-Control-Request-Method",
        "Access-Control-Request-Headers",
        "Content-Type",
      ])
      .allow_methods(vec!["POST", "DELETE"]),
  );

  thread::spawn(move || {
    let game_loop = GameLoop::new(10, 5).unwrap();
    let clients = clients.clone();
    let room = room.clone();

    loop {
      for action in game_loop.actions() {
        match action {
          FrameAction::Tick => {
            runtime::Builder::new().build().unwrap().block_on(async {
              game::main_loop(&room, &clients).await;
            });
          }
          FrameAction::Render { interpolation: _ } => {}
        }
      }
    }
  });

  warp::serve(routes).run(([127, 0, 0, 1], 8000)).await;
}

fn with_clients(clients: Clients) -> impl Filter<Extract = (Clients,), Error = Infallible> + Clone {
  warp::any().map(move || clients.clone())
}

fn with_room(room: Room) -> impl Filter<Extract = (Room,), Error = Infallible> + Clone {
  warp::any().map(move || room.clone())
}
