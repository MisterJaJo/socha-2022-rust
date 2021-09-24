use crate::util::coordinates::Coordinates;
use crate::util::error::Error;
use crate::xml::data::{conversion::FromDeserializable, enums::PlayerTeam, server::state::State};

use super::{board::Board, moves::Move};

#[derive(Debug, Clone)]
pub struct GameState {
    pub start_team: PlayerTeam,
    pub board: Board,

    pub last_move: Option<Move>,
    pub turn: i32,
}

impl GameState {
    pub fn can_perform_move(&self, r#move: &Move, team: PlayerTeam) -> bool {
        // Check if position and target position of the move are valid
        let coords_from = r#move.from.clone();
        let coords_to = r#move.to.clone();

        if !coords_to.in_bounds() || !coords_from.in_bounds() {
            return false;
        }

        // Check if the moved piece belongs to the target team
        let piece_at_position = self.board.get_piece_at(coords_from);
        let piece_belongs_to_team = match piece_at_position {
            Some(piece) => {
                piece.team == team
            }
            None => {
                false
            }
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
        self.board.pieces.iter().for_each(|piece| {
            let player_team = &piece.team;
            let offsets = piece.piece_type.calculate_offsets(player_team);
            for offset in offsets.iter() {
                let new_move = Move {
                    from: piece.coordinates.clone(),
                    to: piece.coordinates.clone() + offset.clone(),
                };

                if self.can_perform_move(&new_move, team.clone()) {
                    moves.push(new_move);
                }
            }
        });

        moves
    }
}

impl FromDeserializable<'_, State> for GameState {
    // The XML protocol is weird
    fn from_deserializable(serializable: &State) -> Result<Self, Error> {
        let deserialized_start_team = &serializable.start_team;
        let deserialized_board = &serializable.board;
        let board = Board::from_deserializable(deserialized_board)?;

        let start_team = &deserialized_start_team.team;

        let last_move = serializable.last_move.as_ref().map(|last_move| {
            let from = Coordinates::from(&last_move.from);
            let to = Coordinates::from(&last_move.to);
            Move { from, to }
        });

        let turn = serializable.turn;

        Ok(Self {
            start_team: start_team.clone(),
            board,
            last_move,
            turn,
        })
    }
}
