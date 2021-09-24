extern crate quick_xml;
extern crate serde;

use serde::Deserialize;

use crate::util::coordinates::Coordinates;

#[derive(Debug, PartialEq, Deserialize, Clone)]
#[serde(rename_all = "UPPERCASE")]
pub enum PlayerTeam {
    One,
    Two,
}

impl PlayerTeam {
    pub fn opponent(&self) -> Self {
        match self {
            PlayerTeam::One => PlayerTeam::Two,
            PlayerTeam::Two => PlayerTeam::One,
        }
    }
}

#[derive(Debug, PartialEq, Deserialize, Clone)]
#[serde(rename_all = "UPPERCASE")]
pub enum PieceColor {
    Blue = 1,
    Red = 2,
}

#[derive(Debug, PartialEq, Deserialize, Clone)]
pub enum PieceType {
    Herzmuschel,
    Moewe,
    Seestern,
    Robbe,
}

impl PieceType {
    pub fn calculate_offsets(&self, player_team: &PlayerTeam) -> Vec<Coordinates> {
        match self {
            PieceType::Herzmuschel => match player_team {
                PlayerTeam::One => vec![
                    Coordinates::new(1, -1),
                    Coordinates::new(1, 1),
                ],
                PlayerTeam::Two => vec![
                    Coordinates::new(-1, -1),
                    Coordinates::new(-1, 1),
                ],
            },
            PieceType::Moewe => match player_team {
                PlayerTeam::One => vec![
                    Coordinates::new(0, -1),
                    Coordinates::new(1, 0),
                    Coordinates::new(0, 1),
                    Coordinates::new(-1, 0),
                ],
                PlayerTeam::Two => vec![
                    Coordinates::new(0, -1),
                    Coordinates::new(1, 0),
                    Coordinates::new(0, 1),
                    Coordinates::new(-1, 0),
                ],
            },
            PieceType::Seestern => match player_team {
                PlayerTeam::One => vec![
                    Coordinates::new(1, 0),

                    Coordinates::new(1, -1),
                    Coordinates::new(1, 1),
                    Coordinates::new(-1, -1),
                    Coordinates::new(-1, 1),
                ],
                PlayerTeam::Two => vec![
                    Coordinates::new(-1, 0),

                    Coordinates::new(1, -1),
                    Coordinates::new(1, 1),
                    Coordinates::new(-1, -1),
                    Coordinates::new(-1, 1),

                ],
            },
            PieceType::Robbe => match player_team {
                PlayerTeam::One => vec![
                    Coordinates::new(-1, -2),
                    Coordinates::new(1, -2),

                    Coordinates::new(2, -1),
                    Coordinates::new(2, 1),

                    Coordinates::new(1, 2),
                    Coordinates::new(-1, 2),

                    Coordinates::new(-2, 1),
                    Coordinates::new(-2, -1),
                ],
                PlayerTeam::Two => vec![
                    Coordinates::new(-1, -2),
                    Coordinates::new(1, -2),

                    Coordinates::new(2, -1),
                    Coordinates::new(2, 1),

                    Coordinates::new(1, 2),
                    Coordinates::new(-1, 2),

                    Coordinates::new(-2, 1),
                    Coordinates::new(-2, -1),
                ],
            },
        }
    }
}

#[derive(Debug, PartialEq, Deserialize, Clone)]
pub enum DataClass {
    #[serde(rename = "welcomeMessage")]
    WelcomeMessage,
    #[serde(rename = "memento")]
    Memento,
    #[serde(rename = "moveRequest")]
    MoveRequest,
    #[serde(rename = "result")]
    Result,
    #[serde(rename = "error")]
    Error,
}
