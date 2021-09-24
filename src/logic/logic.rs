use std::process::exit;

use rand::{seq::SliceRandom, thread_rng};

use crate::game::{game_state::GameState, moves::Move};
use crate::protocol::{
    manager::ProtocolManager,
    message::{ClientSideMessage, ServerSideMessage},
};
use crate::xml::enums::PlayerTeam;

pub struct Logic {
    pub current_game_state: Option<GameState>,
    pub room_id: Option<String>,
    pub last_move: Option<Move>,

    pub own_team: Option<PlayerTeam>,
}

impl Logic {
    pub fn new() -> Self {
        Self {
            current_game_state: None,
            room_id: None,
            last_move: None,
            own_team: None,
        }
    }

    fn calculate_move(&self) -> Option<Move> {
        match &self.current_game_state {
            Some(game_state) => {
                let own_team = &self.own_team;

                match own_team {
                    Some(team) => {
                        let possible_moves = game_state.calculate_possible_moves(team);
                        let mut rng = thread_rng();
                        let sent_move = possible_moves.choose(&mut rng);
                        sent_move.cloned()
                    }
                    None => None,
                }
            }
            None => None,
        }
    }

    fn process_move_request(&mut self, protocol_manager: &mut ProtocolManager) {
        let calculated_move = self.calculate_move();
        log::info!("Calculated move: {:?}", calculated_move);
        if let Some(sent_move) = calculated_move {
            let state_room_id = self.room_id.as_ref().unwrap();
            let room_id = String::from(state_room_id);
            let message = ClientSideMessage::Move { sent_move, room_id };
            if let Err(error) = protocol_manager.send_client_side_message(message) {
                log::error!("Error while trying to send move: {:?}", error);
                exit(0);
            }
        }
    }

    pub fn process_server_side_message(
        &mut self,
        protocol_manager: &mut ProtocolManager,
        message: ServerSideMessage,
    ) {
        match message {
            ServerSideMessage::Left => {
                return;
            }
            ServerSideMessage::MoveRequest => {
                self.process_move_request(protocol_manager);
            }
            ServerSideMessage::Memento { game_state } => {
                self.current_game_state = Some(game_state);
            }
            ServerSideMessage::Result { result } => {
                log::info!("Result: {:?}", result);
                exit(0);
            }
            ServerSideMessage::WelcomeMessage { room_id, own_team } => {
                self.room_id = Some(room_id);
                self.own_team = own_team;

                log::info!("Own team: {:?}", self.own_team);
            }
            ServerSideMessage::Error => {
                log::error!("Error message from server!");
                exit(0);
            }
        }
    }
}
