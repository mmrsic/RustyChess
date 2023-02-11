use std::collections::HashMap;
use std::ops::{Add, Mul};

use bracket_lib::prelude::Point;

use crate::chess_game::ChessGame;
use crate::chessboard::BoardSquare;
use crate::move_rules::*;
use crate::pieces::*;

impl ChessGame {
    /** All possible moves a given piece can currently make in this [ChessGame]. */
    pub fn possible_moves(&self, piece: &Piece) -> Vec<Move> {
        self.possible_targets(piece)
            .iter()
            .map(|target| Move {
                piece: piece.clone(),
                target: *target,
            })
            .filter(|chess_move| !self.calculate_move(chess_move).is_chess_color(piece.color))
            .collect()
    }

    /** All possible target squares of a given piece. */
    fn possible_targets(&self, piece: &Piece) -> Vec<BoardSquare> {
        let mut result = Vec::new();
        piece_deltas(piece).iter().for_each(|piece_delta| {
            let mut distance = 1;
            while distance <= piece_delta.max_distance {
                let delta_candidate = piece_delta.delta.mul(Point::new(distance, distance));
                match self
                    .board
                    .get_square_relative(piece.square, &delta_candidate)
                {
                    None => break,
                    Some(target_square) => match self.piece_at(target_square.position()) {
                        None => result.push(*target_square),
                        Some(other_piece) => {
                            if other_piece.color != piece.color {
                                result.push(*target_square)
                            }
                            break; // Don't allow pieces to jump over other pieces
                        }
                    },
                }
                distance += 1;
            }
        });
        let castling_moves = self.possible_castling_moves(piece);
        castling_moves
            .iter()
            .map(|castling_move| castling_move.target)
            .for_each(|square| result.push(square));
        result
    }

    /** All possible castling moves for a given piece. Empty if specified piece is not a king or
    there are no castling moves possible for the king.*/
    fn possible_castling_moves(&self, king: &Piece) -> Vec<Move> {
        let mut result = Vec::new();
        if king.piece_type != PieceType::King || self.has_already_moved(king) {
            return result;
        }
        [Direction::E, Direction::W]
            .iter()
            .filter_map(|dir| self.next_piece_in_direction(&king.square, dir))
            .filter(|other_piece| other_piece.piece_type == PieceType::Rook)
            .filter(|rook| !self.has_already_moved(rook))
            .for_each(|rook| {
                let rook_x = rook.square.position().x;
                let king_x = king.square.position().x;
                let factor = if rook_x > king_x { 1 } else { -1 };
                let delta = Point::new(2 * factor, 0);
                if let Some(target_square) = self.board.get_square_relative(king.square, &delta) {
                    result.push(Move::new(*king, *target_square))
                }
            });
        result
    }

    /** All [Piece]s able to move to a given target square. */
    pub fn square_challengers(&self, square: &BoardSquare) -> Vec<Piece> {
        let result = self
            .square_context(square)
            .iter()
            .filter_map(|(_, piece)| *piece)
            .filter(|piece| {
                self.possible_targets(&piece)
                    .iter()
                    .any(|&target| target.position() == square.position())
            })
            .collect();
        result
    }

    /** Mapping from [Direction] to [Piece] which is reachable by any piece from a given [BoardSquare]. */
    pub fn square_context(&self, square: &BoardSquare) -> HashMap<Direction, Option<Piece>> {
        let mut result: HashMap<Direction, Option<Piece>> = HashMap::new();
        Direction::all().iter().for_each(|direction| {
            let opt_piece = self.next_piece_in_direction(square, direction);
            result.insert(direction.clone(), opt_piece);
        });
        return result;
    }

    /** The closest piece from a given board square into a given direction. */
    fn next_piece_in_direction(&self, square: &BoardSquare, dir: &Direction) -> Option<Piece> {
        let mut coord = square.position().add(dir.delta());
        let repeatable_directions = Direction::adjacent();
        while self.board.square_at(coord).is_some()
            && self.piece_at(coord).is_none()
            && repeatable_directions.contains(dir)
        {
            coord = coord.add(dir.delta())
        }
        self.piece_at(coord).cloned()
    }

    /** Return a clone of the game where a given move is executed without affecting the original game, */
    pub fn calculate_move(&self, chess_move: &Move) -> ChessGame {
        let mut result = self.clone();
        result.execute_move(&chess_move);
        result
    }
}
