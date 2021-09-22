extern crate log;

mod args;
mod networking;
mod protocol;
mod util;
mod xml;
mod game;
mod logic;

use std::error::Error;

use flexi_logger::{Duplicate, FileSpec, Logger};
use protocol::manager::*;

use crate::{args::client::ClientArgs, logic::logic::process_server_side_message};

fn game_loop(protocol_manager : &mut ProtocolManager) {
    // Wait for a join response from the server
    let joined_response = protocol_manager.wait_for_joined_response();
    match joined_response {
        Ok(room_id) => {
            log::info!("Joined game: {}", room_id);
            
            // Main protocol loop
            
            loop {
                let protocol_message = protocol_manager.get_next_message();
                match protocol_message {
                    Ok(message) => {
                        log::debug!("Message: {:?}", message);
                        process_server_side_message(message);
                    },
                    Err(e) => {
                        log::error!("Error while trying to get/parse next message from the server: {:?}", e);
                        return;
                    },
                }
            }
        },
        Err(e) => {
            log::error!("Error while trying to join: {:?}", e);
            return;
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let file_spec = FileSpec::default().directory("log/");
    Logger::try_with_str("debug")?
        .log_to_file(file_spec)
        .duplicate_to_stderr(Duplicate::All)
        .start()?;

    let collected_args = ClientArgs::collect();
    let opt_protocol_manager = ProtocolManager::from_args(collected_args);
    match opt_protocol_manager {
        Ok(mut protocol_manager) => match protocol_manager.join_game() {
            ProtocolStatus::Success(_) => {
                log::info!("Starting game loop...");
                game_loop(&mut protocol_manager);
            }
            ProtocolStatus::Failure(e) => {
                log::error!("{:?}", e);
            }
        },
        Err(e) => {
            log::error!("{:?}", e);
        }
    }

    Ok(())
}
