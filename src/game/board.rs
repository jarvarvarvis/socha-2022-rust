use crate::{game::piece::Piece, util::coordinates::Coordinates};

pub struct Board {
    pub pieces: Vec<Piece>
}

impl Board {
    pub fn get_piece_at(&self, coords: Coordinates) -> Option<Piece> {
        self.pieces.iter().find(|p| {
            p.coordinates == coords
        })
    }
}