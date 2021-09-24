extern crate log;

mod args;
mod game;
mod logic;
mod networking;
mod protocol;
mod util;
mod xml;

use protocol::manager::*;
use util::{error::Error, logger_setup::setup_logger};

use crate::{args::client::ClientArgs, logic::logic::Logic};

fn game_loop(protocol_manager: &mut ProtocolManager) -> Result<(), Error> {
    // Wait for a join response from the server
    let room_id = protocol_manager.wait_for_joined_response()?;
    log::info!("Joined game: {}", room_id);

    // Main protocol loop
    let mut logic = Logic::new();

    loop {
        let message = protocol_manager.get_next_message()?;
        match logic.process_server_side_message(protocol_manager, message) {
            logic::logic::ClientState::Running => {}
            logic::logic::ClientState::ShouldTerminate => {
                return Ok(());
            }
        }
    }
}

fn main() -> Result<(), Error> {
    setup_logger()?;

    let collected_args = ClientArgs::collect()?;
    let mut protocol_manager = ProtocolManager::from_args(collected_args)?;
    protocol_manager.join_game()?;
    log::info!("Starting game loop...");
    game_loop(&mut protocol_manager)?;
    Ok(())
}
