use serde::{Deserialize, Serialize};

pub trait ToSerializable<To> 
where To: Serialize {
    fn to_serializable(&self) -> To;
}

pub trait FromSerializable<'de, From>
where From: Deserialize<'de> {
    fn from_serializable(serializable: &From) -> Self;
}
