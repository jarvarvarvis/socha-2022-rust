use crate::game::r#move::Move;
use crate::{xml::data::server::data::Received};
use crate::game::game_state::GameState;

pub enum ClientSideMessage {
    JoinAnyGame,
    JoinPreparedGame(String),
    Move(Move)
}

#[derive(Debug)]
pub enum ServerSideMessage {
    Left,
    MoveRequest,
    Memento(GameState),
    Result(Result)
}

impl ClientSideMessage {
    pub fn to_xml(&self) -> String {
        match self {
            ClientSideMessage::JoinAnyGame => {
                String::from("<protocol><join />")
            },
            ClientSideMessage::JoinPreparedGame(reservation) => {
                format!("<protocol><joinPrepared reservationCode\"{}\" />", reservation)
            }
            ClientSideMessage::Move(r#move) => {
                format!("<")
            },
        }
    }
}

impl From<Received> for ServerSideMessage {
    fn from(received: Received) -> Self {
        ServerSideMessage::Left
    }
}