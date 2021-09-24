use crate::util::coordinates::Coordinates;

#[derive(Debug, Clone)]
pub struct Move {
    pub from: Coordinates,
    pub to: Coordinates,
}
