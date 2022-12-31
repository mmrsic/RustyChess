use bracket_lib::geometry::Point;

use crate::chessboard::Chessboard;
use crate::move_rules::Move;
use crate::pieces::*;

pub struct ChessGame {
    pub board: Chessboard,
    pub pieces: Vec<Piece>,
}

impl ChessGame {
    pub fn new() -> Self {
        Self {
            board: Chessboard::new(),
            pieces: create_start_positions(),
        }
    }

    pub fn piece_at(&self, point: Point) -> Option<&Piece> {
        self.pieces
            .iter()
            .find(|p| p.position.x() == point.x as i8 && p.position.y() == point.y as i8)
    }

    pub fn execute_move(&self, move_to_execute: Move) {
        println!("Execute move: {:?}", move_to_execute);
    }
}
