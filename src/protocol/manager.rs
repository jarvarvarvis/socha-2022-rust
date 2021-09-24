extern crate quick_xml;
extern crate serde;

use std::convert::TryFrom;

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
            Ok(size) => Ok(size),
            Err(error) => Err(error),
        }
    }

    pub fn join_game(&mut self) -> Result<usize, Error> {
        let args_reservation = &self.client_args.reservation;
        match args_reservation {
            Some(reservation) => {
                let res_clone = reservation.clone();
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
        let condition_function = |string_buffer: &String| {
            return string_buffer.ends_with("</room>") || string_buffer.contains("<left roomId=\"");
        };
        let text = self
            .network_manager
            .read_string_until_condition(&condition_function)?;

        let text = format!("<received>{}</received>", text);
        let received = from_str::<Received>(&text)?;
        let server_side_message = ServerSideMessage::try_from(received)?;
        Ok(server_side_message)
    }
}
