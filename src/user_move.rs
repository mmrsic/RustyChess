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
}
