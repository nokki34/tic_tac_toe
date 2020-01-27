use crate::actors::ws::WsMessage;
use actix::prelude::Recipient;
use serde_derive::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub struct Match {
  pub id: Uuid,
  pub player1: Uuid,
  pub player2: Option<Uuid>,
}

impl Match {
  pub fn new(id: Uuid, player1: Uuid) -> Match {
    Match {
      id,
      player1,
      player2: None,
    }
  }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ClientMatch {
  pub id: Uuid,
  pub player1: String,
  pub player2: Option<String>,
}

#[derive(Debug, Clone)]
pub struct User {
  pub name: String,
  pub recipient: Recipient<WsMessage>,
}
