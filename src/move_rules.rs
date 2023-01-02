use bracket_lib::prelude::Point;

use crate::chess_game::ChessGame;
use crate::chessboard::{BoardSquare, Chessboard};
use crate::pieces::Piece;

/** A trait which denotes a single move within a game of Chess. */
pub trait ChessGameMove {
    /** Execute the [ChessGameMove]. */
    fn execute(&self, game: &mut ChessGame);
}

/** A single Chess game move of a piece onto an empty target field. */
#[derive(Debug, Clone)]
pub struct Move {
    pub piece: Piece,
    pub target: BoardSquare,
}

impl Move {
    pub fn new(piece: Piece, target: BoardSquare) -> Self {
        Self { piece, target }
    }
}

#[derive(Debug, Clone)]
pub struct CapturingMove {
    pub attacker: Piece,
    pub victim: Piece,
}

impl CapturingMove {
    pub fn new(attacker: Piece, victim: Piece) -> Self {
        Self { attacker, victim }
    }
}

impl ChessGameMove for Move {
    fn execute(&self, game: &mut ChessGame) {
        println!("Execute move: {:?}", self);
        game.pieces.iter_mut().for_each(|bp| {
            if bp.position == self.piece.position {
                bp.position.clone_from(&self.target);
            }
        });
        println!("Num pieces: {}", game.pieces.len())
    }
}

impl ChessGameMove for CapturingMove {
    fn execute(&self, game: &mut ChessGame) {
        println!("Execute move: {:?}", self);
        game.pieces.retain(|bp| bp.position != self.victim.position);
        Move::new(self.attacker.clone(), self.victim.position).execute(game);
    }
}

/*

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
