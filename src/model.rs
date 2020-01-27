use crate::actors::ws::WsMessage;
use actix::prelude::Recipient;
use uuid::Uuid;

#[derive(Debug, Clone, Copy)]
pub struct Match {
  pub player1: Uuid,
  pub player2: Option<Uuid>,
}

impl Match {
  pub fn new(player1: Uuid) -> Match {
    Match {
      player1,
      player2: None,
    }
  }
}

#[derive(Debug, Clone)]
pub struct User {
  pub name: String,
  pub recipient: Recipient<WsMessage>,
}
