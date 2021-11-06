use crate::util::coordinates::Coordinates;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Move {
    pub from: Coordinates,
    pub to: Coordinates,
}
