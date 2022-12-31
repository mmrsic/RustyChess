use bracket_lib::prelude::Point;

use crate::chessboard::{BoardSquare, Chessboard};
use crate::pieces::Piece;

#[derive(Debug)]
pub struct Move {
    pub piece: Piece,
    pub target: BoardSquare,
}

/*

pub struct CapturingMove {
    pub attacker: Piece,
    pub victim: Piece,
}

pub struct EnPassantMove {
    pub attacker: Piece,
    pub victim: Piece,
    pub target: BoardSquare,
}

pub struct CastlingMove {
    pub king: Piece,
    pub rook: Piece,
}

*/

pub fn create_basic_possible_moves(piece: &Piece, board: &Chessboard) -> Vec<Move> {
    let mut result = Vec::new();

    for delta_x in -1..=1 {
        for delta_y in -1..=1 {
            if delta_x != 0 || delta_y != 0 {
                let delta = Point::new(delta_x, delta_y);
                if let Some(optional_square) = board.get_square_relative(piece.position, delta) {
                    result.push(Move {
                        piece: piece.clone(),
                        target: *optional_square,
                    });
                }
            }
        }
    }

    return result;
}
