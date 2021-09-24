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
            Ok(stream) => Ok(NetworkManager { stream }),
            Err(error) => Err(Error::IOError(error)),
        }
    }

    pub fn write_text(&mut self, text: &str) -> Result<usize, Error> {
        let text_bytes = text.as_bytes();
        let write_result = self.stream.write(&text_bytes);

        match write_result {
            Ok(size) => Ok(size),
            Err(error) => Err(Error::IOError(error)),
        }
    }

    fn read_raw_exact(&mut self, length: usize) -> Result<Vec<u8>, Error> {
        let mut read_buffer = vec![0; length];
        let read_result = self.stream.read(&mut read_buffer);

        match read_result {
            Ok(_) => Ok(read_buffer),
            Err(error) => Err(Error::IOError(error)),
        }
    }

    pub fn read_string_exact(&mut self, length: usize) -> Result<String, Error> {
        match self.read_raw_exact(length) {
            Ok(read_buffer) => {
                let from_utf8 = String::from_utf8(read_buffer);
                match from_utf8 {
                    Ok(s) => Ok(s),
                    Err(e) => Err(Error::FromUtf8Error(e)),
                }
            }
            Err(error) => Err(error),
        }
    }

    pub fn read_string_until_condition(
        &mut self,
        condition_function: ConditionFunction,
    ) -> Result<String, Error> {
        let mut string_buffer = String::with_capacity(READ_BUFFER_SIZE * 3);

        loop {
            let read_result = self.read_string_exact(READ_BUFFER_SIZE);
            match read_result {
                Ok(owned_string) => {
                    let string = owned_string.trim_matches(char::from(0));

                    string_buffer.push_str(string);
                    if condition_function(&string_buffer) {
                        return Ok(string_buffer);
                    }
                }
                Err(error) => return Err(error),
            }
        }
    }
}
