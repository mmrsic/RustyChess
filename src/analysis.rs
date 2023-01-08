use std::collections::{HashMap, HashSet};
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
                let target_piece = self.piece_at(target_square.position());
                (None == target_piece || target_piece.unwrap().color != piece.color)
                    && (piece.piece_type != PieceType::King
                        || self
                            .square_contesters(&target_square)
                            .iter()
                            .filter(|p| p.square != piece.square)
                            .all(|p| p.color == piece.color))
            })
            .map(|target| Move {
                piece: piece.clone(),
                target: *target,
            })
            .collect()
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

    /** All [BoardSquare]s currently threatened by the pieces (applied to a given filter) of this [ChessGame].  */
    pub fn piece_controlled_area<G>(&self, piece_filter: G) -> HashSet<BoardSquare>
    where
        G: FnMut(&&Piece) -> bool,
    {
        let mut result = HashSet::new();
        self.pieces.iter().filter(piece_filter).for_each(|piece| {
            self.possible_moves(piece).iter().for_each(|possible_move| {
                result.insert(possible_move.target);
            })
        });
        return result;
    }
}
