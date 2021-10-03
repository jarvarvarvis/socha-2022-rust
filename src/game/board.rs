use std::collections::HashMap;
use std::fmt::{Display, Formatter};

use crate::xml::enums::{PieceType, PlayerTeam};
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
    pub fn get_piece_at(&self, coords: &Coordinates) -> Option<&Piece> {
        self.pieces.get(coords)
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

    pub fn get_pieces_for_team(&self, team: PlayerTeam) -> impl Iterator<Item = &Piece> {
        self.pieces.values().filter(move |piece| piece.team == team)
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

impl Display for Board {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> std::fmt::Result {
        for y in 0..8 {
            for x in 0..8 {
                let coords = Coordinates { x, y };
                let piece = self.get_piece_at(&coords);
                match piece {
                    Some(piece) => {
                        let piece_identifier = match piece.piece_type {
                            PieceType::Herzmuschel => "H",
                            PieceType::Moewe => "M",
                            PieceType::Seestern => "S",
                            PieceType::Robbe => "R",
                        };
                        let team_identifier = match piece.team {
                            PlayerTeam::One => "1",
                            PlayerTeam::Two => "2",
                        };

                        write!(fmt, "{}{} ", piece_identifier, team_identifier)?;
                    },
                    None => {
                        write!(fmt, "-- ")?;
                    }
                }
            }
            write!(fmt, "\n")?;
        }

        Ok(())
    }
}
