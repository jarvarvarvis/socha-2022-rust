extern crate quick_xml;
extern crate serde;

use crate::networking::manager::NetworkManager;
use crate::util::error::Error;
use crate::xml::serialization::from_str;
use crate::xml::server::data::Received;
use crate::{args::client::ClientArgs, xml::server::data::Joined};

use super::message::{ClientSideMessage, ServerSideMessage};

pub struct ProtocolManager {
    network_manager: NetworkManager,
    client_args: ClientArgs,
}

impl ProtocolManager {
    fn from_valid_args(client_args: ClientArgs) -> Result<Self, Error> {
        let host = client_args.host.clone();
        let port = client_args.port;
        let network_manager = NetworkManager::connect(host, port)?;
        Ok(Self {
            network_manager,
            client_args: client_args.clone(),
        })
    }

    pub fn from_args(client_args: ClientArgs) -> Result<Self, Error> {
        Self::from_valid_args(client_args)
    }

    pub fn send_client_side_message(&mut self, message: ClientSideMessage) -> Result<usize, Error> {
        let text = message.to_xml()?;

        match self.network_manager.write_text(&text) {
            Ok(s) => Ok(s),
            Err(e) => Err(e),
        }
    }

    pub fn join_game(&mut self) -> Result<usize, Error> {
        let reservation = &self.client_args.reservation;
        match reservation {
            Some(res) => {
                let res_clone = res.clone();
                self.send_client_side_message(ClientSideMessage::JoinPreparedGame {
                    reservation: res_clone,
                })
            }
            None => self.send_client_side_message(ClientSideMessage::JoinAnyGame),
        }
    }

    pub fn wait_for_joined_response(&mut self) -> Result<String, Error> {
        let response = self.network_manager.read_string_exact(69)?; // nice
        if !response.starts_with("<protocol>") {
            return Err(Error::SimpleError(String::from(
                "Response didn't start with <protocol>!",
            )));
        }

        let formatted_response = response.replace("<protocol>\n", "");

        let joined = from_str::<Joined>(&formatted_response)?;
        Ok(joined.room_id)
    }

    pub fn get_next_message(&mut self) -> Result<ServerSideMessage, Error> {
        let condition_function = |s: &String| {
            return s.ends_with("</room>") || s.contains("<left roomId=\"");
        };
        let text = self
            .network_manager
            .read_string_until_condition(&condition_function)?;

        let text = format!("<received>{}</received>", text);
        let received = from_str::<Received>(&text)?;
        let server_side_message = ServerSideMessage::from(received);
        Ok(server_side_message)
    }
}
