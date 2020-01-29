use crate::actors::server::{
  Connect, CreateMatch, Disconnect, JoinMatch, ListMatches, TicTacToeServer,
};
use crate::model::{ClientMatch, ClientUser};

use actix::prelude::*;
use actix_web_actors::ws;
use names::{Generator, Name};
use serde_derive::{Deserialize, Serialize};
use std::time::{Duration, Instant};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum WsDto {
  CreateMatch,
  JoinMatch(Uuid),
  ListMatches,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type", content = "data")]
pub enum WsResponseDto {
  ListMatchesResponse(Vec<ClientMatch>),
  LoginResponse(ClientUser),
}

#[derive(Message, Debug, Serialize, Deserialize)]
#[rtype(result = "()")]
pub struct WsMessage(WsDto);

#[derive(Clone)]
pub struct WsSession {
  id: Uuid,
  tic_tac_toe_server: Addr<TicTacToeServer>,
  hb: Instant,
}

impl Actor for WsSession {
  type Context = ws::WebsocketContext<Self>;

  fn started(&mut self, ctx: &mut Self::Context) {
    let mut generator = Generator::with_naming(Name::Plain); // Should be probably later moved out
    self.hb(ctx);

    self
      .tic_tac_toe_server
      .send(Connect {
        id: self.id,
        name: generator.next().unwrap(),
        addr: ctx.address().recipient(),
      })
      .into_actor(self)
      .then(|res, _, ctx| {
        match res {
          Ok(resp) => {
            let resp = WsResponseDto::LoginResponse(resp);
            let resp = serde_json::to_string(&resp).unwrap();
            ctx.text(resp);
          }
          Err(e) => {
            println!("Mailbox error, {:?}", e);
          }
        }
        fut::ready(())
      })
      .wait(ctx);
  }

  fn stopping(&mut self, _: &mut Self::Context) -> Running {
    self.tic_tac_toe_server.do_send(Disconnect { id: self.id });
    Running::Stop
  }
}

impl WsSession {
  pub fn new(tic_tac_toe_server: Addr<TicTacToeServer>) -> WsSession {
    WsSession {
      id: Uuid::new_v4(),
      hb: Instant::now(),
      tic_tac_toe_server,
    }
  }

  fn hb(&self, ctx: &mut ws::WebsocketContext<Self>) {
    ctx.run_interval(Duration::from_secs(5), |act, ctx| {
      if Instant::now().duration_since(act.hb) > Duration::from_secs(10) {
        println!("Websocket Client heartbeat failed, disconnecting!");
        act.tic_tac_toe_server.do_send(Disconnect { id: act.id });
        ctx.stop();
        return;
      }
      ctx.ping(b"");
    });
  }

  fn handle_ws_dto(&self, msg: WsDto, ctx: &mut ws::WebsocketContext<Self>) {
    match msg {
      WsDto::CreateMatch => self
        .tic_tac_toe_server
        .do_send(CreateMatch { user_id: self.id }),
      WsDto::ListMatches => self
        .tic_tac_toe_server
        .send(ListMatches)
        .into_actor(self)
        .then(|res, _, ctx| {
          match res {
            Ok(resp) => {
              let resp = WsResponseDto::ListMatchesResponse(resp);
              let resp = serde_json::to_string(&resp).unwrap();
              ctx.text(resp);
            }
            Err(e) => {
              println!("Something is wrong, {:?}", e);
            }
          }
          fut::ready(())
        })
        .wait(ctx),
      WsDto::JoinMatch(id) => self.tic_tac_toe_server.do_send(JoinMatch {
        match_id: id,
        user_id: self.id,
      }),
    };
  }
}

impl Handler<WsMessage> for WsSession {
  type Result = ();

  fn handle(&mut self, msg: WsMessage, ctx: &mut Self::Context) {
    let contents = serde_json::to_string(&msg).unwrap();
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

    match msg {
      ws::Message::Ping(msg) => {
        self.hb = Instant::now();
        ctx.pong(&msg);
      }
      ws::Message::Pong(_) => {
        self.hb = Instant::now();
      }
      ws::Message::Text(msg) => {
        // println!("Trying to parse: {}", msg);
        let msg = serde_json::from_str::<WsMessage>(&msg);
        let msg = match msg {
          Err(err) => {
            eprintln!("Failed to parse message {:?}", err);
            return;
          }
          Ok(msg) => msg,
        };
        // println!("Successfully parsed message {:?}", &msg);
        self.handle_ws_dto(msg.0, ctx);
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
