use crate::actors::ws::WsMessage;
use crate::model::{Match, User};
use actix::prelude::*;
use std::cell::RefCell;
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Message)]
#[rtype(result = "()")]
pub struct Connect {
  pub id: Uuid,
  pub name: String,
  pub addr: Recipient<WsMessage>,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct Disconnect {
  pub id: Uuid,
}

#[derive(Message)]
#[rtype(result = "Vec<Match>")]
pub struct ListMatches;

#[derive(Message)]
#[rtype(result = "(Result<(), CreateMatchError>)")]
pub struct CreateMatch {
  user_id: Uuid,
}

#[derive(Message)]
#[rtype(result = "(Result<(), JoinMatchError>)")]
pub struct JoinMatch {
  match_id: Uuid,
  user_id: Uuid,
}

#[derive(Clone)]
pub struct TicTacToeServer {
  users: HashMap<Uuid, User>,
  matches: HashMap<Uuid, RefCell<Match>>,
}

pub enum CreateMatchError {
  NoSuchUser,
}

pub enum JoinMatchError {
  NoSuchMatch,
  NoSuchUser,
}

impl TicTacToeServer {
  pub fn new() -> TicTacToeServer {
    TicTacToeServer {
      matches: HashMap::new(),
      users: HashMap::new(),
    }
  }

  fn create_match(&mut self, msg: CreateMatch) -> Result<(), CreateMatchError> {
    let user = &self.users.get(&msg.user_id);

    match user {
      Some(_) => {
        let new_match = Match::new(msg.user_id);
        self.matches.insert(Uuid::new_v4(), RefCell::new(new_match));
        Ok(())
      }
      None => Err(CreateMatchError::NoSuchUser),
    }
  }

  fn join_match(&mut self, msg: JoinMatch) -> Result<(), JoinMatchError> {
    let match_to_join = &self
      .matches
      .get(&msg.match_id)
      .ok_or(JoinMatchError::NoSuchMatch)?;

    self
      .matches
      .get(&msg.user_id)
      .ok_or(JoinMatchError::NoSuchUser)?;

    match_to_join.borrow_mut().player2 = Some(msg.user_id);

    Ok(())
  }

  fn list_available_matches(&self) -> Vec<Match> {
    self
      .matches
      .values()
      .filter_map(|val| {
        let existing_match = val.borrow();
        match existing_match.player2 {
          Some(_) => Some(*existing_match),
          None => None,
        }
      })
      .collect()
  }
}

impl Actor for TicTacToeServer {
  type Context = Context<Self>;
}

impl Handler<Connect> for TicTacToeServer {
  type Result = ();

  fn handle(&mut self, msg: Connect, _: &mut Context<Self>) {
    println!("Someone joined");
    self.users.insert(
      msg.id,
      User {
        name: msg.name,
        recipient: msg.addr,
      },
    );
  }
}

impl Handler<Disconnect> for TicTacToeServer {
  type Result = ();

  fn handle(&mut self, msg: Disconnect, _: &mut Context<Self>) {
    let user = self.users.remove(&msg.id).unwrap();
    println!("User \"{}\" disconnected", user.name);
    // TODO: should probably disconnect from matches a user is in
  }
}

impl Handler<ListMatches> for TicTacToeServer {
  type Result = MessageResult<ListMatches>;

  fn handle(&mut self, _: ListMatches, _: &mut Context<Self>) -> Self::Result {
    MessageResult(self.list_available_matches())
  }
}

impl Handler<CreateMatch> for TicTacToeServer {
  type Result = MessageResult<CreateMatch>;

  fn handle(&mut self, msg: CreateMatch, _: &mut Context<Self>) -> Self::Result {
    MessageResult(self.create_match(msg))
  }
}

impl Handler<JoinMatch> for TicTacToeServer {
  type Result = MessageResult<JoinMatch>;

  fn handle(&mut self, msg: JoinMatch, _: &mut Context<Self>) -> Self::Result {
    MessageResult(self.join_match(msg))
  }
}
