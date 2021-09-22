use std::io::{Read, Write};
use std::net::TcpStream;

use crate::util::error::Error;

pub struct NetworkManager {
    stream: TcpStream,
}

pub const READ_BUFFER_SIZE: usize = 256;

type ConditionFunction<'a> = &'a dyn Fn(&String) -> bool;

impl NetworkManager {
    pub fn connect(host: String, port: i32) -> Result<Self, Error> {
        let addr = format!("{}:{}", host, port);
        let stream = TcpStream::connect(addr);
        match stream {
            Ok(s) => Ok(NetworkManager { stream: s }),
            Err(e) => Err(Error::IOError(e)),
        }
    }

    pub fn write_text(&mut self, text: &str) -> Result<usize, Error> {
        let buf = text.as_bytes();
        let write_result = self.stream.write(&buf);

        match write_result {
            Ok(s) => Ok(s),
            Err(e) => Err(Error::IOError(e)),
        }
    }

    fn read_raw_exact(&mut self, length: usize) -> Result<Vec<u8>, Error> {
        let mut buf = vec![0; length];
        let read_result = self.stream.read(&mut buf);

        match read_result {
            Ok(_) => Ok(buf),
            Err(e) => Err(Error::IOError(e)),
        }
    }

    pub fn read_string_exact(&mut self, length: usize) -> Result<String, Error> {
        match self.read_raw_exact(length) {
            Ok(vec) => {
                let from_utf8 = String::from_utf8(vec);
                match from_utf8 {
                    Ok(s) => Ok(s),
                    Err(e) => Err(Error::FromUtf8Error(e)),
                }
            }
            Err(e) => Err(e),
        }
    }

    pub fn read_string_until_condition(&mut self, condition_function: ConditionFunction) -> Result<String, Error> {
        let mut string_cache = String::with_capacity(READ_BUFFER_SIZE * 3);

        loop {
            let read_result = self.read_string_exact(READ_BUFFER_SIZE);
            match read_result {
                Ok(owned_string) => {
                    let string = owned_string.trim_matches(char::from(0));

                    string_cache.push_str(string);
                    if condition_function(&string_cache) {
                        return Ok(string_cache);
                    }
                }
                Err(e) => return Err(e),
            }
        }
    }
}
