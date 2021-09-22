extern crate serde;
extern crate quick_xml;

use serde::Deserialize;

#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum PlayerTeam {
    One,
    Two
}

#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum PieceColor {
    Blue = 1,
    Red = 2
}

#[derive(Debug, PartialEq, Deserialize)]
pub enum PieceType {
    Herzmuschel,
    Moewe,
    Seestern,
    Robbe
}
