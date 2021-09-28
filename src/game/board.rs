use crate::xml::server::state::Board as XmlBoard;
use crate::{
    game::piece::Piece,
    util::{coordinates::Coordinates, error::Error},
    xml::conversion::FromDeserializable,
};

#[derive(Debug, Clone)]
pub struct Board {
    pub pieces: Vec<Piece>,
}

impl Board {
    pub fn get_piece_at(&self, coords: Coordinates) -> Option<&Piece> {
        self.pieces.iter().find(|piece| piece.coordinates == coords)
    }
}

impl FromDeserializable<'_, XmlBoard> for Board {
    fn from_deserializable(deserializable: &XmlBoard) -> Result<Self, Error> {
        let deserialized_pieces = &deserializable.pieces.entries;

        let pieces = deserialized_pieces
            .iter()
            .map(Piece::from_deserializable)
            .filter_map(|piece| piece.ok())
            .collect();

        Ok(Board { pieces })
    }
}
