use crate::actors::ws::WsMessage;
use crate::model::{ClientMatch, ClientUser, Match, User};
use actix::prelude::*;
use std::cell::RefCell;
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Message)]
#[rtype(result = "ClientUser")]
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
#[rtype(result = "Vec<ClientMatch>")]
pub struct ListMatches;

#[derive(Message)]
#[rtype(result = "(Result<(), CreateMatchError>)")]
pub struct CreateMatch {
  pub user_id: Uuid,
}

#[derive(Message)]
#[rtype(result = "(Result<(), JoinMatchError>)")]
pub struct JoinMatch {
  pub match_id: Uuid,
  pub user_id: Uuid,
}

#[derive(Clone)]
pub struct TicTacToeServer {
  pub users: HashMap<Uuid, User>,
  pub matches: HashMap<Uuid, RefCell<Match>>,
}

#[derive(Debug)]
pub enum CreateMatchError {
  NoSuchUser,
}

#[derive(Debug)]
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
        let id = Uuid::new_v4();
        let new_match = Match::new(id, msg.user_id);
        self.matches.insert(id, RefCell::new(new_match));
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

  fn list_available_matches(&self) -> Vec<ClientMatch> {
    self
      .matches
      .values()
      .filter_map(|val| {
        let existing_match = val.borrow();

        // Should say that the user has disconnected
        if let Some(player1) = &self.users.get(&existing_match.player1) {
          return match existing_match.player2 {
            Some(_) => None,
            None => Some(ClientMatch {
              id: existing_match.id,
              player1: ClientUser {
                id: existing_match.player1,
                name: player1.name.clone(),
              },
              player2: None,
            }),
          };
        };
        None
      })
      .collect()
  }
}

impl Actor for TicTacToeServer {
  type Context = Context<Self>;
}

impl Handler<Connect> for TicTacToeServer {
  type Result = MessageResult<Connect>;

  fn handle(&mut self, msg: Connect, _: &mut Context<Self>) -> Self::Result {
    println!("{} joined", msg.name);
    self.users.insert(
      msg.id,
      User {
        name: msg.name.clone(),
        recipient: msg.addr,
      },
    );
    MessageResult(ClientUser {
      id: msg.id,
      name: msg.name,
    })
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
