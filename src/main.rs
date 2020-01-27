mod actors;
mod model;

use actix::{Actor, Addr};
use actix_web::{web, App, Error, HttpRequest, HttpResponse, HttpServer};
use actix_web_actors::ws;
use actors::server::TicTacToeServer;
use actors::ws::WsSession;

async fn tic_tac_toe_route(
    req: HttpRequest,
    stream: web::Payload,
    server: web::Data<Addr<TicTacToeServer>>,
) -> Result<HttpResponse, Error> {
    ws::start(WsSession::new(server.get_ref().clone()), &req, stream)
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    println!("Tic Tac Toe server is starting...");
    let server = TicTacToeServer::new().start();

    HttpServer::new(move || {
        App::new()
            .data(server.clone())
            .service(web::resource("/game/").to(tic_tac_toe_route))
    })
    .bind("0.0.0.0:8000")?
    .run()
    .await
}
