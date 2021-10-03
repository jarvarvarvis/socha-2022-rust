use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use crate::util::coordinates::Coordinates;
use crate::util::error::Error;
use crate::xml::{conversion::FromDeserializable, enums::PlayerTeam, server::state::State};

use super::{board::Board, moves::Move};

#[derive(Debug, Clone)]
pub struct GameState {
    pub start_team: PlayerTeam,
    pub board: Board,

    pub last_move: Option<Move>,
    pub turn: u32,

    pub ambers: (i32, i32),
}

impl GameState {
    pub fn get_current_team(&self) -> PlayerTeam {
        self.start_team.next_n(self.turn)
    }

    pub fn get_ambers_for(&self, team: PlayerTeam) -> i32 {
        match team {
            PlayerTeam::One => self.ambers.0,
            PlayerTeam::Two => self.ambers.1,
        }
    }

    fn increment_ambers_for(&mut self, team: PlayerTeam) {
        match team {
            PlayerTeam::One => self.ambers.0 += 1,
            PlayerTeam::Two => self.ambers.1 += 1
        }
    }

    /// This doesn't seem to be reliable in the normal game.
    /// It looks like the server doesn't send the last game state (when one of the players wins).
    ///
    /// I recommend to only use this when performing moves manually.
    pub fn get_result(&self) -> Option<PlayerTeam> {
        if self.turn >= 59 {
            return match self.ambers {
                (2, 0) | (2, 1) => Some(PlayerTeam::One),
                (0, 2) | (1, 2) => Some(PlayerTeam::Two),
                (0, 0) | (1, 1) => {
                    // TODO: check positions of minor pieces?
                    None
                }
                (_, _) => None,
            };
        }

        match self.ambers {
            (2, 0) | (2, 1) => Some(PlayerTeam::One),
            (0, 2) | (1, 2) => Some(PlayerTeam::Two),
            (1, 1) => {
                // TODO: check positions of minor pieces?
                None
            }
            (_, _) => None,
        }
    }

    pub fn can_perform_move(&self, r#move: &Move, team: PlayerTeam) -> bool {
        // Check if the team is equal to the current team
        if team != self.get_current_team() {
            return false;
        }

        // Check if position and target position of the move are valid
        let coords_from = r#move.from.clone();
        let coords_to = r#move.to.clone();

        if !coords_to.in_bounds() || !coords_from.in_bounds() {
            return false;
        }

        // Check if the moved piece belongs to the target team
        let piece_at_position = self.board.get_piece_at(coords_from);
        let piece_belongs_to_team = match piece_at_position {
            Some(piece) => piece.team == team,
            None => false,
        };

        // Check if the piece moves to an empty field or a field
        // that contains an opponent piece
        let piece_at_target = self.board.get_piece_at(coords_to);
        let move_to_valid_field = match piece_at_target {
            Some(piece) => piece.team != team,
            None => true,
        };

        piece_belongs_to_team && move_to_valid_field
    }

    pub fn calculate_possible_moves(&self, team: &PlayerTeam) -> Vec<Move> {
        let mut moves = Vec::new();
        for entry in self.board.pieces.iter() {
            let coordinates = entry.0;
            let piece = entry.1;

            let player_team = &piece.team;
            let offsets = piece.piece_type.calculate_offsets(player_team);
            for offset in offsets.iter() {
                let new_move = Move {
                    from: coordinates.clone(),
                    to: coordinates.clone() + offset.clone(),
                };

                if self.can_perform_move(&new_move, team.clone()) {
                    moves.push(new_move);
                }
            }
        }

        moves
    }

    fn advance(&mut self) {
        self.turn += 1;
    }

    pub fn perform_move(&mut self, r#move: &Move) -> Result<(), Error> {
        let team = self.get_current_team();
        if !self.can_perform_move(r#move, team.clone()) {
            return Err(Error::SimpleError(
                String::from("The move couldn't be performed.")
            ))
        }

        let move_from = r#move.from.clone();
        let move_to = &r#move.to;
        
        // Update the ambers count and remove the moved piece if the piece at the target position is stacked.
        // Increment the count of the moved piece if the piece at the target position is not already stacked.
        let opt_piece_at_target = self.board.get_piece_at(move_to.clone());
        let mut should_set_moved_piece_stacked = false;
        let mut should_remove_moved_piece = false;
        
        if let Some(piece_at_target) = opt_piece_at_target {
            if piece_at_target.is_stacked() {
                self.increment_ambers_for(team);
                should_remove_moved_piece = true;
            } else {
                should_set_moved_piece_stacked = true;
            }
        }
        
        // Update the position of the moved piece
        self.board.move_piece(&move_from, move_to);

        // Set piece stacked if necessary
        if should_set_moved_piece_stacked {
            let opt_moved_piece = self.board.get_piece_at_ref_mut(&move_from);
            if let Some(moved_piece) = opt_moved_piece {
                moved_piece.count = 2;
            }
        }

        // Remove the piece if necessary
        if should_remove_moved_piece {
            self.board.pieces.remove(&move_from);
        }
        
        // Advance the GameState
        self.advance();
        
        // Set last move
        let cloned_move = r#move.clone();
        self.last_move = Some(cloned_move);

        Ok(())
    }
}

impl FromDeserializable<'_, State> for GameState {
    fn from_deserializable(deserializable: &State) -> Result<Self, Error> {
        let deserialized_start_team = &deserializable.start_team;
        let deserialized_board = &deserializable.board;
        let board = Board::from_deserializable(deserialized_board)?;

        let start_team = &deserialized_start_team.team;

        let last_move = deserializable.last_move.as_ref().map(|last_move| {
            let from = Coordinates::from(&last_move.from);
            let to = Coordinates::from(&last_move.to);
            Move { from, to }
        });

        let turn = deserializable.turn;

        let mut team_one_ambers = 0;
        let mut team_two_ambers = 0;

        for amber in deserializable.ambers.entries.iter() {
            match amber.team.team {
                PlayerTeam::One => {
                    team_one_ambers = amber.int.value;
                }
                PlayerTeam::Two => {
                    team_two_ambers = amber.int.value;
                }
            }
        }

        let ambers = (team_one_ambers, team_two_ambers);

        Ok(Self {
            start_team: start_team.clone(),
            board,
            last_move,
            turn,
            ambers,
        })
    }
}
