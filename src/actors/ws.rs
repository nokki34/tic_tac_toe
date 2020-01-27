use crate::actors::server::{Connect, Disconnect, TicTacToeServer};
use actix::prelude::*;
use actix_web_actors::ws;
use serde_derive::{Deserialize, Serialize};
use std::time::{Duration, Instant};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub enum WsServerMessages {
  CreateMatch,
  JoinMatch,
  ListMatches,
}

#[derive(Serialize, Deserialize)]
pub enum WsMessages {
  Server(WsServerMessages),
  MatchSession(()),
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct WsMessage(WsMessages);

#[derive(Clone)]
pub struct WsSession {
  id: Uuid,
  ticTacToeServer: Addr<TicTacToeServer>,
  hb: Instant,
}

impl Actor for WsSession {
  type Context = ws::WebsocketContext<Self>;

  fn started(&mut self, ctx: &mut Self::Context) {
    self.hb(ctx);

    self.ticTacToeServer.do_send(Connect {
      id: self.id,
      name: "test".to_string(),
      addr: ctx.address().recipient(),
    })
  }

  fn stopping(&mut self, _: &mut Self::Context) -> Running {
    self.ticTacToeServer.do_send(Disconnect { id: self.id });
    Running::Stop
  }
}

impl WsSession {
  pub fn new(ticTacToeServer: Addr<TicTacToeServer>) -> WsSession {
    WsSession {
      id: Uuid::new_v4(),
      hb: Instant::now(),
      ticTacToeServer,
    }
  }

  fn hb(&self, ctx: &mut ws::WebsocketContext<Self>) {
    ctx.run_interval(Duration::from_secs(5), |act, ctx| {
      if Instant::now().duration_since(act.hb) > Duration::from_secs(10) {
        println!("Websocket Client heartbeat failed, disconnecting!");
        act.ticTacToeServer.do_send(Disconnect { id: act.id });
        ctx.stop();
        return;
      }
      ctx.ping(b"");
    });
  }
}

impl Handler<WsMessage> for WsSession {
  type Result = ();

  fn handle(&mut self, msg: WsMessage, ctx: &mut Self::Context) {
    let contents = serde_json::to_string(&msg.0).unwrap();
    ctx.text(contents);
  }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WsSession {
  fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
    let msg = match msg {
      Err(_) => {
        ctx.stop();
        println!("Got a protocol error");
        return;
      }
      Ok(msg) => msg,
    };

    println!("{:?}", msg);

    match msg {
      ws::Message::Ping(msg) => {
        self.hb = Instant::now();
        ctx.pong(&msg);
      }
      ws::Message::Pong(_) => {
        self.hb = Instant::now();
      }
      ws::Message::Text(msg) => {
        // let msg: WsMessage = serde_json::from_str(&msg).unwrap();
      }
      ws::Message::Binary(_) => {
        eprintln!("Unexpected binary");
      }
      ws::Message::Close(_) => {
        ctx.stop();
      }
      ws::Message::Continuation(_) => {
        ctx.stop();
      }
      _ => (),
    }
  }
}
