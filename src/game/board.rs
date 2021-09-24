use crate::xml::data::server::state::Board as XmlBoard;
use crate::{
    game::piece::Piece,
    util::{coordinates::Coordinates, error::Error},
    xml::data::conversion::FromDeserializable,
};

#[derive(Debug, Clone)]
pub struct Board {
    pub pieces: Vec<Piece>,
}

impl Board {
    pub fn get_piece_at(&self, coords: Coordinates) -> Option<&Piece> {
        self.pieces.iter().find(|p| p.coordinates == coords)
    }
}

impl FromDeserializable<'_, XmlBoard> for Board {
    fn from_deserializable(serializable: &XmlBoard) -> Result<Self, Error> {
        let deserialized_pieces = &serializable.pieces.entries;
        
        let pieces = deserialized_pieces
            .iter()
            .map(|piece_entry| {
                let from_deserialized_coordinates =
                    Coordinates::from_deserializable(&piece_entry.coordinates);
                let coordinates = from_deserialized_coordinates.unwrap();

                let piece = &piece_entry.piece;

                let piece_type = piece.piece_type.clone();
                let team = piece.team.clone();
                let count = piece.count;

                Piece {
                    piece_type,
                    team,
                    coordinates,
                    count,
                }
            })
            .collect();

        Ok(Board { pieces })
    }
}
