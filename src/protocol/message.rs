use std::convert::TryFrom;

use crate::game::game_state::GameState;
use crate::game::moves::Move;
use crate::game::result::GameResult;
use crate::util::error::Error;
use crate::xml::conversion::FromDeserializable;
use crate::xml::enums::{DataClass, PlayerTeam};
use crate::xml::server::data::Received;

pub enum ClientSideMessage {
    JoinAnyGame,
    JoinPreparedGame { reservation: String },
    Move { sent_move: Move, room_id: String },
}

#[derive(Debug)]
pub enum ServerSideMessage {
    Error,
    WelcomeMessage {
        room_id: String,
        own_team: Option<PlayerTeam>,
    },
    Left,
    MoveRequest,
    Memento {
        game_state: GameState,
    },
    Result {
        result: GameResult,
    },
}

impl ClientSideMessage {
    pub fn to_xml(&self) -> Result<String, Error> {
        match self {
            ClientSideMessage::JoinAnyGame => Ok(String::from("<protocol><join />")),
            ClientSideMessage::JoinPreparedGame { reservation } => Ok(format!(
                "<protocol><joinPrepared reservationCode\"{}\" />",
                reservation
            )),
            ClientSideMessage::Move { sent_move, room_id } => {
                let from = &sent_move.from;
                let to = &sent_move.to;

                let from_declaration = format!("<from x=\"{}\" y=\"{}\"/>", from.x, from.y);
                let to_declaration = format!("<to x=\"{}\" y=\"{}\"/>", to.x, to.y);

                Ok(format!(
                    "<room roomId=\"{}\"><data class=\"move\">{}{}</data></room>",
                    room_id, from_declaration, to_declaration
                ))
            }
        }
    }
}

impl TryFrom<Received> for ServerSideMessage {
    type Error = Error;

    fn try_from(received: Received) -> Result<Self, Error> {
        match received.left {
            Some(_) => return Ok(ServerSideMessage::Left),
            None => {}
        }

        let room = &received.rooms[0];
        let room_data = &room.data;
        match room_data.class {
            DataClass::WelcomeMessage => {
                let room_id = &room.room_id;
                let own_team = &room.data.color;
                Ok(ServerSideMessage::WelcomeMessage {
                    room_id: String::from(room_id),
                    own_team: own_team.clone(),
                })
            }
            DataClass::Memento => {
                let unwrapped_state = room_data.state.as_ref().unwrap();
                let game_state_conversion_result = GameState::from_deserializable(&unwrapped_state);
                let game_state = game_state_conversion_result.unwrap();
                Ok(ServerSideMessage::Memento { game_state })
            }
            DataClass::MoveRequest => Ok(ServerSideMessage::MoveRequest),
            DataClass::Result => {
                let result = GameResult::from_deserializable(room_data)?;
                Ok(ServerSideMessage::Result { result })
            }
            DataClass::Error => Ok(ServerSideMessage::Error),
        }
    }
}
