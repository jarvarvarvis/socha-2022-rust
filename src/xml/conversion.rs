use serde::{Deserialize, Serialize};

use crate::util::error::Error;

pub trait ToSerializable<To>
where
    To: Serialize,
{
    fn to_serializable(&self) -> To;
}

pub trait FromDeserializable<'de, From>
where
    Self: Sized,
    From: Deserialize<'de>,
{
    fn from_deserializable(deserializable: &From) -> Result<Self, Error>;
}
