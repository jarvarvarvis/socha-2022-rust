use quick_xml::de::from_str as quick_xml_from_str;
use serde::de::DeserializeOwned;

use crate::util::error::Error;

pub fn from_str<T: DeserializeOwned>(string: &str) -> Result<T, Error> {
    match quick_xml_from_str::<T>(string) {
        Ok(deserialized) => Ok(deserialized),
        Err(error) => Err(Error::XmlDeserializeError(error)),
    }
}
