use crate::chessboard::BoardSquare;
use crate::move_rules::Move;
use crate::pieces::Piece;

#[derive(Debug)]
pub struct UserMove {
    pub piece: Piece,
    pub possible_moves: Vec<Move>,
}

impl UserMove {
    pub fn new(piece: Piece, possible_moves: Vec<Move>) -> Self {
        Self {
            piece,
            possible_moves,
        }
    }
    pub fn move_to_target(&self, target: BoardSquare) -> Option<&Move> {
        self.possible_moves
            .iter()
            .find(|possible_move| possible_move.target == target)
    }
}
