use std::collections::HashMap;

use crate::xml::server::state::Board as XmlBoard;
use crate::{
    game::piece::Piece,
    util::{coordinates::Coordinates, error::Error},
    xml::conversion::FromDeserializable,
};

#[derive(Debug, Clone)]
pub struct Board {
    pub pieces: HashMap<Coordinates, Piece>,
}

impl Board {
    pub fn get_piece_at(&self, coords: Coordinates) -> Option<&Piece> {
        self.pieces.get(&coords)
    }

    pub fn move_piece(&mut self, from: &Coordinates, to: &Coordinates) {
        if let Some(piece) = self.pieces.remove(from) {
            self.pieces.insert(to.clone(), piece);
        }
    }

    pub fn get_piece_at_mut<'a>(&'a mut self, coords: Coordinates) -> Option<&'a mut Piece> {
        self.pieces.get_mut(&coords)
    }

    pub fn get_piece_at_ref_mut<'a>(&'a mut self, coords: &Coordinates) -> Option<&'a mut Piece> {
        let coords = coords.clone();
        self.get_piece_at_mut(coords)
    }
}

impl FromDeserializable<'_, XmlBoard> for Board {
    fn from_deserializable(deserializable: &XmlBoard) -> Result<Self, Error> {
        let deserialized_pieces = &deserializable.pieces.entries;

        let mut pieces: HashMap<Coordinates, Piece> = HashMap::new();

        for piece in deserialized_pieces.iter() {
            let coordinates = Coordinates::from_deserializable(&piece.coordinates)?;
            let piece = Piece::from_deserializable(piece)?;

            pieces.insert(coordinates, piece);
        }

        Ok(Board { pieces })
    }
}
