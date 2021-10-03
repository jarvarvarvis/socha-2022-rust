use crate::util::coordinates::Coordinates;
use crate::util::error::Error;
use crate::xml::conversion::FromDeserializable;
use crate::xml::enums::{MINOR_PIECES, PieceType, PlayerTeam};
use crate::xml::server::state::PiecesEntry as XmlPiece;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Piece {
    pub piece_type: PieceType,
    pub team: PlayerTeam,
    pub coordinates: Coordinates,

    pub count: i32,
}

impl Piece {
    pub fn is_stacked(&self) -> bool {
        self.count == 2
    }

    pub fn is_minor_piece(&self) -> bool {
        MINOR_PIECES.contains(&self.piece_type)
    }

    pub fn dist_from_starting_line(&self) -> i32 {
        match self.team {
            PlayerTeam::One => {
                self.coordinates.x
            },
            PlayerTeam::Two => {
                7 - self.coordinates.x
            },
        }
    }
}

impl FromDeserializable<'_, XmlPiece> for Piece {
    fn from_deserializable(deserializable: &XmlPiece) -> Result<Self, Error> {
        let coordinates = Coordinates::from_deserializable(&deserializable.coordinates)?;

        let piece = &deserializable.piece;

        let piece_type = piece.piece_type.clone();
        let team = piece.team.clone();
        let count = piece.count;

        Ok(Piece {
            piece_type,
            team,
            coordinates,
            count,
        })
    }
}
