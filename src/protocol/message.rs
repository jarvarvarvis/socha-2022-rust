use quick_xml::se::to_string;

use crate::game::game_state::GameState;
use crate::game::moves::Move;
use crate::game::result::GameResult;
use crate::util::error::Error;
use crate::xml::data::conversion::{FromDeserializable, ToSerializable};
use crate::xml::data::enums::DataClass;
use crate::xml::data::server::data::Received;

pub enum ClientSideMessage {
    JoinAnyGame,
    JoinPreparedGame { reservation: String },
    Move { sent_move: Move, room_id: String },
}

#[derive(Debug)]
pub enum ServerSideMessage {
    WelcomeMessage { room_id: String },
    Left,
    MoveRequest,
    Memento { game_state: GameState },
    Result { result: GameResult },
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
                let serializable_move = sent_move.to_serializable();
                let serialization_result = to_string(&serializable_move);

                match serialization_result {
                    Ok(serialized_move) => Ok(format!(
                        "<room roomId=\"{}\"><data class=\"move\">{}</data></room>",
                        room_id, serialized_move
                    )),
                    Err(error) => Err(Error::XmlDeserializeError(error)),
                }
            }
        }
    }
}

impl From<Received> for ServerSideMessage {
    fn from(received: Received) -> Self {
        match received.left {
            Some(_) => {
                return ServerSideMessage::Left;
            },
            None => { },
        }

        let room = &received.rooms[0];
        let room_data = &room.data;
        match room_data.class {
            DataClass::WelcomeMessage => {
                let room_id = &room.room_id;
                ServerSideMessage::WelcomeMessage { room_id: room_id.to_string() }
            },
            DataClass::Memento => {
                let unwrapped_state = room_data.state.as_ref().unwrap();
                let game_state_conversion_result = GameState::from_deserializable(&unwrapped_state);
                let game_state = game_state_conversion_result.unwrap();
                ServerSideMessage::Memento { game_state }
            },
            DataClass::MoveRequest => {
                ServerSideMessage::MoveRequest
            },
            DataClass::Result => {
                let result_conversion_result = GameResult::from_deserializable(room_data);
                match result_conversion_result {
                    Ok(result) => ServerSideMessage::Result { result },
                    Err(e) => todo!("Oh no")
                }
            },
        }
    }
}
