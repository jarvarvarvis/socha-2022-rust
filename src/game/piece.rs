use crate::util::coordinates::Coordinates;
use crate::xml::conversion::FromDeserializable;
use crate::xml::enums::{PieceType, PlayerTeam};
use crate::xml::server::state::PiecesEntry as XmlPiece;

#[derive(Debug, Clone)]
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
}

impl FromDeserializable<'_, XmlPiece> for Piece {
    fn from_deserializable(deserializable: &XmlPiece) -> Result<Self, crate::util::error::Error> {
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
