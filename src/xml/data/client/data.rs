extern crate serde;
extern crate quick_xml;

use serde::Serialize;

#[derive(Debug, Serialize, PartialEq)]
pub struct From {
    pub x: i32,
    pub y: i32
}

#[derive(Debug, Serialize, PartialEq)]
pub struct To {
    pub x: i32,
    pub y: i32
}

#[derive(Debug, Serialize, PartialEq)]
pub struct Move {
    pub from: From,
    pub to: To
}
