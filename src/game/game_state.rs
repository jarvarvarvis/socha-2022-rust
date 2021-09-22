use crate::xml::data::{enums::PlayerTeam};

use super::{board::Board, r#move::Move};

pub struct GameState {
    pub start_team: PlayerTeam,
    pub board: Board,
    
    pub last_move: Move,

}