use std::cmp::Ordering;

use crate::util::coordinates::Coordinates;
use crate::util::error::Error;
use crate::xml::{conversion::FromDeserializable, enums::PlayerTeam, server::state::State};

use super::piece::Piece;
use super::{board::Board, moves::Move};

#[derive(Debug, Clone)]
pub struct GameState {
    pub start_team: PlayerTeam,
    pub board: Board,

    pub last_move: Option<Move>,
    pub turn: u32,

    pub ambers: (i32, i32),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GameStateResult {
    Player(PlayerTeam),
    Draw,
    Nothing,
}

impl GameState {
    pub fn get_current_team(&self) -> PlayerTeam {
        self.start_team.next_n(self.turn)
    }

    fn increment_ambers_for(&mut self, team: PlayerTeam) {
        match team {
            PlayerTeam::One => self.ambers.0 += 1,
            PlayerTeam::Two => self.ambers.1 += 1,
        }
    }

    fn piece_dist(coordinates: &Coordinates, piece: &Piece) -> i32 {
        let diff = coordinates.x - piece.team.start_line();
        diff.abs()
    }

    fn evaluate_light_piece_positions(&self) -> GameStateResult {
        let pieces = &self.board.pieces;
        let team_one_positions = pieces
            .iter()
            .filter(|entry| entry.1.team == PlayerTeam::One)
            .filter(|entry| entry.1.is_light_piece())
            .map(|entry| Self::piece_dist(entry.0, entry.1));

        let team_two_positions = pieces
            .iter()
            .filter(|entry| entry.1.team == PlayerTeam::Two)
            .filter(|entry| entry.1.is_light_piece())
            .map(|entry| Self::piece_dist(entry.0, entry.1));

        let mut team_one_positions_sorted = team_one_positions.collect::<Vec<i32>>();
        team_one_positions_sorted.sort_by(|a, b| b.cmp(a));

        let mut team_two_positions_sorted = team_two_positions.collect::<Vec<i32>>();
        team_two_positions_sorted.sort_by(|a, b| b.cmp(a));

        let mut team_two_iter = team_two_positions_sorted.iter();
        for team_one_position in team_one_positions_sorted.iter() {
            if let Some(team_two_position) = team_two_iter.next() {
                let compare_result = team_one_position.cmp(team_two_position);
                match compare_result {
                    Ordering::Greater => {
                        return GameStateResult::Player(PlayerTeam::One)
                    },
                    Ordering::Less => {
                        return GameStateResult::Player(PlayerTeam::Two)
                    },
                    Ordering::Equal => {
                        continue;
                    },
                }
            }
        }

        GameStateResult::Draw
    }

    pub fn get_result(&self) -> GameStateResult {
        if self.turn >= 59 {
            return match self.ambers {
                (1, 0) | (2, _) | (3, _) => GameStateResult::Player(PlayerTeam::One),
                (0, 1) | (_, 2) | (_, 3) => GameStateResult::Player(PlayerTeam::Two),
                (0, 0) | (1, 1) => self.evaluate_light_piece_positions(),
                (_, _) => GameStateResult::Nothing,
            };
        }

        match self.ambers {
            (2, _) | (3, _) => GameStateResult::Player(PlayerTeam::One),
            (_, 2) | (_, 3) => GameStateResult::Player(PlayerTeam::Two),
            (_, _) => GameStateResult::Nothing,
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
        let piece_at_position = self.board.get_piece_at(&coords_from);
        let piece_belongs_to_team = match piece_at_position {
            Some(piece) => piece.team == team,
            None => false,
        };

        // Check if the piece moves to an empty field or a field
        // that contains an opponent piece
        let piece_at_target = self.board.get_piece_at(&coords_to);
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
            return Err(Error::SimpleError(String::from(
                "The move couldn't be performed.",
            )));
        }

        let move_from = r#move.from.clone();
        let move_to = r#move.to.clone();

        if let Some(piece_at_target) = self.board.get_piece_at(&move_to) {
            // If the piece at the target position is stacked:
            // - remove it
            // - remove own piece
            // - increment ambers count for own team
            if piece_at_target.is_stacked() {
                self.board.pieces.remove(&move_to);
                self.board.pieces.remove(&move_from);
                self.increment_ambers_for(team);
            }
            // If the piece at the target position is not stacked:
            // - remove it
            // - move own piece to the target position
            // - make own piece stacked
            else {
                self.board.pieces.remove(&move_to);
                self.board.move_piece(&move_from, &move_to);

                if let Some(moved_piece_at_target) = self.board.get_piece_at_ref_mut(&move_to) {
                    moved_piece_at_target.count = 2;
                }
            }
        }
        // If there is no piece at the target:
        // - move own piece to the target position
        else {
            // Update the position of the moved piece
            self.board.move_piece(&move_from, &move_to);
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
