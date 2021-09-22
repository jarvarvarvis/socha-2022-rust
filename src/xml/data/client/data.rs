extern crate serde;
extern crate quick_xml;

use serde::Serialize;

#[derive(Debug, Serialize, PartialEq)]
pub struct Move {
    // TODO?
}

#[derive(Debug, Serialize, PartialEq)]
pub struct Data {
    class : String,
    sent_move : Move
}

#[derive(Debug, Serialize, PartialEq)]
pub struct Room {
    #[serde(rename = "roomId")]
    room_id : String
}
