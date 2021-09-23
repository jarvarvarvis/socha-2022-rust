use std::ops::{Add, Sub};

use crate::xml::data::server::state::{From as XmlFrom, To as XmlTo};
use crate::xml::data::conversion::FromDeserializable;

use crate::xml::data::server::state::Coordinates as XmlCoordinates;

use super::error::Error;

#[derive(Debug, PartialEq, Eq)]
pub struct Vec2<T> {
    pub x: T,
    pub y: T
}

pub type Coordinates = Vec2<i32>;

impl Add for Coordinates {
    type Output = Coordinates;

    fn add(self, other: Self) -> Self::Output {
        Coordinates {
            x: self.x + other.x,
            y: self.y + other.y
        }
    }
}

impl Sub for Coordinates {
    type Output = Coordinates;

    fn sub(self, other: Self) -> Self::Output {
        Coordinates {
            x: self.x + other.x,
            y: self.y + other.y
        }
    }
}

impl FromDeserializable<'_, XmlCoordinates> for Coordinates {
    fn from_deserializable(serializable: &XmlCoordinates) -> Result<Self, Error> {
        Ok(Self {
            x: serializable.x,
            y: serializable.y
        })
    }
}

impl From<&XmlFrom> for Coordinates {
    fn from(from: &XmlFrom) -> Self {
        Coordinates {
            x: from.x,
            y: from.y,
        }
    }
}

impl From<&XmlTo> for Coordinates {
    fn from(to: &XmlTo) -> Self {
        Coordinates {
            x: to.x,
            y: to.y
        }
    }
}
