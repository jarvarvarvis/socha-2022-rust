extern crate serde;
extern crate quick_xml;

use quick_xml::de::from_str;

use crate::xml::data::server::data::Received;
use crate::{args::client::ClientArgs, xml::data::server::data::Joined};
use crate::networking::manager::NetworkManager;
use crate::util::error::Error;

use super::message::{ClientSideMessage, ServerSideMessage};

pub struct ProtocolManager {
    network_manager: NetworkManager,
    client_args: ClientArgs,
}

pub enum ProtocolStatus {
    Success(usize),
    Failure(Error),
}

impl ProtocolManager {
    fn from_valid_args(client_args: ClientArgs) -> Result<Self, Error> {
        let host = client_args.host.clone();
        let port = client_args.port;
        match NetworkManager::connect(host, port) {
            Ok(network_manager) => Ok(Self {
                network_manager,
                client_args: client_args.clone(),
            }),
            Err(e) => Err(e),
        }
    }

    pub fn from_args(client_args: Result<ClientArgs, Error>) -> Result<Self, Error> {
        match client_args {
            Ok(args) => Self::from_valid_args(args),
            Err(e) => Err(e),
        }
    }

    pub fn send_client_side_message(&mut self, message: ClientSideMessage) -> ProtocolStatus {
        let text = message.to_xml();

        match self.network_manager.write_text(&text) {
            Ok(s) => ProtocolStatus::Success(s),
            Err(e) => ProtocolStatus::Failure(e),
        }
    }

    pub fn join_game(&mut self) -> ProtocolStatus {
        let reservation = &self.client_args.reservation;
        match reservation {
            Some(res) => {
                let res_clone = res.clone();
                self.send_client_side_message(
                    ClientSideMessage::JoinPreparedGame(res_clone)
                )
            },
            None => self.send_client_side_message(ClientSideMessage::JoinAnyGame)
        }
    }

    pub fn wait_for_joined_response(&mut self) -> Result<String, Error> {
        let expected_joined_response = self.network_manager.read_string_exact(69); // nice
        match expected_joined_response {
            Ok(s) => {
                if !s.starts_with("<protocol>") {
                    return Err(Error::SimpleError(String::from("Response didn't start with <protocol>!")));
                }

                let slice = s.replace("<protocol>\n", "");

                let xml_parse_result = from_str::<Joined>(&slice);
                match xml_parse_result {
                    Ok(joined) => {
                        Ok(joined.room_id)
                    },
                    Err(e) => {
                        Err(Error::XmlDeserializeError(e))
                    }
                }
            },
            Err(e) => {
                Err(e)
            }
        }
    }

    pub fn get_next_message(&mut self) -> Result<ServerSideMessage, Error> {
        let condition_function = |s: &String| {
            return s.ends_with("</room>") || s.contains("<left roomId=\"");
        };
        let text = self.network_manager.read_string_until_condition(&condition_function);
        match text {
            Ok(s) => {
                let s = format!("<received>{}</received>", s);
                let xml_parse_result = from_str::<Received>(&s);
                match xml_parse_result {
                    Ok(received) => {
                        let server_side_message = ServerSideMessage::from(received);
                        Ok(server_side_message)
                    },
                    Err(e) => {
                        Err(Error::XmlDeserializeError(e))
                    }
                }
            },
            Err(e) => {
                Err(e)
            }
        }
    }
}
