use crate::util::error::Error;
use crate::xml::conversion::FromDeserializable;
use crate::xml::enums::{LIGHT_PIECES, PieceType, PlayerTeam};
use crate::xml::server::state::PiecesEntry as XmlPiece;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Piece {
    pub piece_type: PieceType,
    pub team: PlayerTeam,

    pub count: i32,
}

impl Piece {
    pub fn is_stacked(&self) -> bool {
        self.count == 2
    }

    pub fn is_light_piece(&self) -> bool {
        LIGHT_PIECES.contains(&self.piece_type)
    }
}

impl FromDeserializable<'_, XmlPiece> for Piece {
    fn from_deserializable(deserializable: &XmlPiece) -> Result<Self, Error> {
        let piece = &deserializable.piece;

        let piece_type = piece.piece_type.clone();
        let team = piece.team.clone();
        let count = piece.count;

        Ok(Piece {
            piece_type,
            team,
            count,
        })
    }
}
