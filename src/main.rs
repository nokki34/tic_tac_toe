mod actors;
mod model;

use actix::prelude::*;
use actix_web_actors::ws::WebsocketContext;
use actors::server::TicTacToeServer;
use actors::ws::WsSession;

#[actix_rt::main]
async fn main() {
    println!("Hello, world!");
}
