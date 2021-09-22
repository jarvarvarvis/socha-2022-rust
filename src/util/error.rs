use std::string::FromUtf8Error;
use std::num::ParseIntError;

use args::ArgsError;
use quick_xml::DeError;

#[derive(Debug)]
pub enum Error {
    ArgsError(ArgsError),
    IOError(std::io::Error),
    ParseIntError(ParseIntError),
    SimpleError(String),
    FromUtf8Error(FromUtf8Error),
    XmlDeserializeError(DeError)
}
