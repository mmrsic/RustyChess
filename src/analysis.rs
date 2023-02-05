use bracket_lib::prelude::Point;
use std::collections::HashMap;
use std::ops::{Add, Mul};

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
                target: **target,
            })
            .filter(|chess_move| !self.calculate_move(chess_move).is_chess_color(piece.color))
            .collect()
    }

    /** All possible target squares of a given piece. */
    fn possible_targets(&self, piece: &Piece) -> Vec<&BoardSquare> {
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
                        None => result.push(target_square),
                        Some(other_piece) => {
                            if other_piece.color != piece.color {
                                result.push(target_square)
                            }
                            break;
                        }
                    },
                }
                distance += 1;
            }
        });
        result
    }

    /** All [Piece]s able to move to a given target square. */
    pub fn square_contesters(&self, square: &BoardSquare) -> Vec<Piece> {
        let result = self
            .square_context(square)
            .iter()
            .filter(|(_, piece)| piece.is_some())
            .map(|(_, piece)| piece.unwrap())
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
        let repeatable_directions = Direction::adjacent();
        let mut result: HashMap<Direction, Option<Piece>> = HashMap::new();
        Direction::all().iter().for_each(|direction| {
            let mut coord = square.position().add(direction.delta());
            while self.board.square_at(coord).is_some()
                && self.piece_at(coord).is_none()
                && repeatable_directions.contains(direction)
            {
                coord = coord.add(direction.delta())
            }
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
