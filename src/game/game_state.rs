use crate::xml::data::{conversion::FromSerializable, enums::PlayerTeam, server::state::State};

use super::{board::Board, r#move::Move};

#[derive(Debug)]
pub struct GameState {
    pub start_team: PlayerTeam,
    pub board: Board,

    pub last_move: Move,
}

impl FromSerializable<'_, State> for GameState {
    fn from_serializable(serializable: &State) -> Self {
        todo!()
    }
}
