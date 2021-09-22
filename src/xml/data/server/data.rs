extern crate quick_xml;
extern crate serde;

use serde::Deserialize;

use crate::xml::data::enums::PlayerTeam;

use super::result::{Definition, Scores, Winner};
use super::state::State;

#[derive(Debug, Deserialize, PartialEq)]
pub struct Data {
    /// Possible values for `class`:
    /// - welcomeMessage
    /// - memento
    /// - moveRequest
    /// - result
    ///
    /// TODO: convert this to an enum?
    pub class: String,

    /// Only available when `class` is equal to `memento`
    pub state: Option<State>,

    /// Only available when `class` is equal to `welcomeMessage`
    pub color: Option<PlayerTeam>,

    /// Only available when `class` is equal to `result`
    pub definition: Option<Definition>,
    /// Only available when `class` is equal to `result`
    pub scores: Option<Scores>,
    /// Only available when `class` is equal to `result`
    pub winner: Option<Winner>,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Room {
    pub data: Data,

    #[serde(rename = "roomId")]
    pub room_id: String,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Left {
    #[serde(rename = "roomId")]
    pub room_id: String,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Received {
    pub left: Option<Left>,

    #[serde(rename = "room", default)]
    pub rooms: Vec<Room>,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Joined {
    #[serde(rename = "roomId")]
    pub room_id: String
}
