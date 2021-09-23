use crate::util::coordinates::Coordinates;
use crate::xml::data::enums::{PieceType, PlayerTeam};

#[derive(Debug)]
pub struct Piece {
    pub piece_type: PieceType,
    pub team: PlayerTeam,
    pub coordinates: Coordinates,

    count: i32,
}

impl Piece {
    pub fn is_stacked(&self) -> bool {
        self.count == 2
    }
}
