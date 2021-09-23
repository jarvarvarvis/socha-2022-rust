use crate::util::coordinates::Coordinates;
use crate::xml::data::client::data::Move as XmlMove;
use crate::xml::data::client::data::{From, To};
use crate::xml::data::conversion::ToSerializable;

#[derive(Debug)]
pub struct Move {
    pub from: Coordinates,
    pub to: Coordinates
}

impl ToSerializable<XmlMove> for Move {
    fn to_serializable(&self) -> XmlMove {
        XmlMove {
            from: From {
                x: self.from.x,
                y: self.from.y,
            },
            to: To {
                x: self.to.x,
                y: self.to.y,
            },
        }
    }
}
