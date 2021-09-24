extern crate quick_xml;
extern crate serde;

use serde::Deserialize;

use crate::xml::enums::{PieceType, PlayerTeam};

#[derive(Debug, Deserialize, PartialEq)]
pub struct StartTeam {
    #[serde(rename = "$value")]
    pub team: PlayerTeam,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Coordinates {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Piece {
    #[serde(rename = "type")]
    pub piece_type: PieceType,
    pub team: PlayerTeam,
    pub count: i32,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct PiecesEntry {
    pub coordinates: Coordinates,
    pub piece: Piece,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Pieces {
    #[serde(rename = "entry", default)]
    pub entries: Vec<PiecesEntry>,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Board {
    pub pieces: Pieces,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct From {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct To {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename = "lastMove")]
pub struct LastMove {
    pub from: From,
    pub to: To,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Team {
    #[serde(rename = "$value")]
    pub team: PlayerTeam,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Int {
    #[serde(rename = "$value")]
    pub value: i32,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct AmbersEntry {
    pub team: Team,
    pub int: Int,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Ambers {
    #[serde(rename = "enum-type")]
    pub enum_type : String,

    #[serde(rename = "entry", default)]
    pub entries: Vec<AmbersEntry>,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct State {
    #[serde(rename = "startTeam")]
    pub start_team: StartTeam,
    pub board: Board,

    #[serde(rename = "lastMove")]
    pub last_move: Option<LastMove>,
    pub ambers: Ambers,

    pub turn: i32,
}
