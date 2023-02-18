use std::collections::HashMap;

use crate::domain::game::*;
use crate::domain::pieces::CapturePolicy::Mandatory;

impl ChessGame {
    /** All possible moves a given piece can currently make in this [ChessGame]. */
    pub fn possible_moves(&self, piece: &Piece) -> Vec<Move> {
        self.possible_targets(piece)
            .iter()
            .map(|target| Move {
                piece: piece.clone(),
                target: *target,
            })
            .filter(|chess_move| !self.is_chess_after_move(chess_move))
            .filter(|chess_move| {
                !is_castling_move(
                    &chess_move.piece,
                    &chess_move.piece.square,
                    &chess_move.target,
                ) || self
                    .board
                    .all_squares_between(chess_move.piece.square, chess_move.target)
                    .iter()
                    .all(|square| !self.is_chess_after_move(&Move::new(*piece, *square)))
            })
            .collect()
    }

    /** All possible target squares of a given piece. */
    fn possible_targets(&self, piece: &Piece) -> Vec<BoardSquare> {
        let mut result = Vec::new();
        piece_deltas(piece).iter().for_each(|piece_delta| {
            let mut distance = 1;
            while distance <= piece_delta.max_distance {
                let coord = (
                    piece_delta.delta.0 * distance,
                    piece_delta.delta.1 * distance,
                );
                match self.board.square_relative(piece.square, coord) {
                    None => break,
                    Some(target_square) => match self.piece_at(target_square.position()) {
                        None => {
                            if piece_delta.capture_policy != Mandatory {
                                result.push(*target_square)
                            } else if let Some(en_passant_target) = self.en_passant_target() {
                                if en_passant_target == target_square {
                                    result.push(*en_passant_target);
                                }
                            }
                        }
                        Some(other_piece) => {
                            if piece_delta.may_capture() && other_piece.color != piece.color {
                                result.push(*target_square)
                            }
                            break; // Don't allow pieces to jump over other pieces
                        }
                    },
                }
                distance += 1;
            }
        });
        self.possible_castling_moves(piece)
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
                let rook_x = rook.square.position().0;
                let king_x = king.square.position().0;
                let factor = if rook_x > king_x { 1 } else { -1 };
                let delta = (2 * factor, 0);
                if let Some(target_square) = self.board.square_relative(king.square, delta) {
                    result.push(Move::new(*king, *target_square))
                }
            });
        result
    }

    /** All [Piece]s able to move to a given target square. */
    pub fn square_challengers(&self, square: &BoardSquare) -> Vec<Piece> {
        self.square_context(square)
            .iter()
            .filter_map(|(_, piece)| *piece)
            .filter(|piece| {
                self.possible_targets(&piece)
                    .iter()
                    .any(|&target| target.position() == square.position())
            })
            .collect()
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
        let pos = square.position();
        let mut coord = (pos.0 + dir.delta().0, pos.1 + dir.delta().1);
        let repeatable_directions = Direction::adjacent();
        while self.board.square_at(coord).is_some()
            && self.piece_at(coord).is_none()
            && repeatable_directions.contains(dir)
        {
            coord = (coord.0 + dir.delta().0, coord.1 + dir.delta().1)
        }
        self.piece_at(coord).cloned()
    }

    /** Return a clone of the game where a given move is executed without affecting the original game, */
    pub fn calculate_move(&self, chess_move: &Move) -> ChessGame {
        let mut result = self.clone();
        result.execute_move(&chess_move);
        result
    }

    /** Check whether a given move when executed leaves the moving piece color in chess. */
    fn is_chess_after_move(&self, chess_move: &Move) -> bool {
        self.calculate_move(chess_move)
            .is_chess_color(chess_move.piece.color)
    }
}
