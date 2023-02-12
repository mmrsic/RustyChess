use crate::domain::chessboard::BoardSquare;
use crate::domain::game::Move;
use crate::domain::pieces::Piece;

/** A user move consists of the piece the user wants to move and all possible moves this piece is
able to execute. */
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

    /** The only possible move to a given target square of the chess board. */
    pub fn possible_move_to_target(&self, target: BoardSquare) -> Option<&Move> {
        self.possible_moves
            .iter()
            .find(|possible_move| possible_move.target == target)
    }
}
