use std::collections::HashMap;
use std::ops::Add;

use crate::chess_game::ChessGame;
use crate::chessboard::BoardSquare;
use crate::move_rules::*;
use crate::pieces::*;

impl ChessGame {
    /** All possible moves a given piece can currently make in this [ChessGame]. */
    pub fn possible_moves(&self, piece: &Piece) -> Vec<Move> {
        piece_deltas(piece)
            .iter()
            .filter_map(|delta| self.board.get_square_relative(piece.square, delta))
            .filter(|target_square| {
                self.is_target_allowed_for_color(piece.color, target_square)
                    && !self.is_in_chess(piece, &target_square)
            })
            .map(|target| Move {
                piece: piece.clone(),
                target: *target,
            })
            .filter(|chess_move| !self.calculate_move(chess_move).is_chess_color(piece.color))
            .collect()
    }

    /** Whether any of the kings of this game is in chess. */
    pub fn is_in_chess(&self, piece: &Piece, target_square: &&&BoardSquare) -> bool {
        piece.piece_type == PieceType::King
            && self
                .square_contesters(&&target_square)
                .iter()
                .any(|p| p.color != piece.color)
    }

    /** All [Piece]s able to move to a given target square. */
    pub fn square_contesters(&self, square: &BoardSquare) -> Vec<Piece> {
        let result = self
            .square_context(square)
            .iter()
            .filter(|(_, piece)| piece.is_some())
            .map(|(_, piece)| piece.unwrap())
            .filter(|piece| {
                piece_deltas(&piece)
                    .iter()
                    .any(|&x| x.add(piece.square.position()) == square.position())
            })
            .collect();
        result
    }

    /** Mapping from [Direction] to [Piece] which is reachable by any piece from a given [BoardSquare]. */
    pub fn square_context(&self, square: &BoardSquare) -> HashMap<Direction, Option<Piece>> {
        let mut result: HashMap<Direction, Option<Piece>> = HashMap::new();
        Direction::all().iter().for_each(|direction| {
            let coord = square.position().add(direction.delta());
            result.insert(direction.clone(), self.piece_at(coord).cloned());
        });
        return result;
    }

    /** Return a clone of the game where a given move is executed without affecting the original game, */
    pub fn calculate_move(&self, chess_move: &Move) -> ChessGame {
        let mut result = self.clone();
        result.execute_move(&chess_move);
        result
    }
}
