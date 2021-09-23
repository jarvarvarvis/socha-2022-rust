use crate::game::piece::Piece;
use crate::util::coordinates::Coordinates;
use crate::util::error::Error;
use crate::xml::data::{conversion::FromDeserializable, enums::PlayerTeam, server::state::State};

use super::{board::Board, moves::Move};

#[derive(Debug)]
pub struct GameState {
    pub start_team: PlayerTeam,
    pub board: Board,

    pub last_move: Move,
}

impl FromDeserializable<'_, State> for GameState {
    // The XML protocol is bullshit
    fn from_deserializable(serializable: &State) -> Result<Self, Error> {
        let deserialized_start_team = &serializable.start_team;
        let deserialized_pieces = &serializable.board.pieces.entries;
        let deserialized_last_move = serializable.last_move.as_ref().unwrap();
        let deserialized_from = &deserialized_last_move.from;
        let deserialized_to = &deserialized_last_move.to;

        let start_team = &deserialized_start_team.team;
        let pieces = deserialized_pieces
            .iter()
            .map(|piece_entry| {
                let from_coordinates = Coordinates::from_deserializable(&piece_entry.coordinates);
                let coordinates = from_coordinates.unwrap();

                let piece = &piece_entry.piece;

                Piece {
                    piece_type: piece.piece_type.clone(),
                    team: piece.team.clone(),
                    coordinates,
                    count: piece.count,
                }
            })
            .collect();

        let board = Board { pieces };

        let from = Coordinates {
            x: deserialized_from.x,
            y: deserialized_from.y,
        };
        let to = Coordinates {
            x: deserialized_to.x,
            y: deserialized_to.y,
        };
        let last_move = Move { from, to };

        Ok(Self {
            start_team: start_team.clone(),
            board,
            last_move
        })
    }
}
