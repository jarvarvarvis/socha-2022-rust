extern crate quick_xml;
extern crate serde;

use serde::Deserialize;

#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum PlayerTeam {
    One,
    Two,
}

#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum PieceColor {
    Blue = 1,
    Red = 2,
}

#[derive(Debug, PartialEq, Deserialize)]
pub enum PieceType {
    Herzmuschel,
    Moewe,
    Seestern,
    Robbe,
}

#[derive(Debug, PartialEq, Deserialize)]
pub enum DataClass {
    #[serde(rename = "welcomeMessage")]
    WelcomeMessage,
    #[serde(rename = "memento")]
    Memento,
    #[serde(rename = "moveRequest")]
    MoveRequest,
    #[serde(rename = "result")]
    Result,
}
