use bracket_lib::prelude::Point;

use crate::chess_game::ChessGame;
use crate::chessboard::BoardSquare;
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
        game.pieces.iter_mut().for_each(|game_piece| {
            if game_piece.square == self.piece.square {
                game_piece.square.clone_from(&self.target);
            }
        });
        println!("Executed move: {:?}", self);
    }
}

impl ChessGameMove for CapturingMove {
    fn execute(&self, game: &mut ChessGame) {
        game.pieces
            .retain(|game_piece| game_piece.square != self.victim.square);
        Move::new(self.attacker.clone(), self.victim.square).execute(game);
        println!("Number of pieces: {:?}", game.pieces.len());
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

#[derive(Eq, PartialEq, Hash, Clone, Debug)]
pub enum Direction {
    N,
    E,
    W,
    S,
    NE,
    NW,
    SE,
    SW,
    NNE,
    NNW,
    SSE,
    SSW,
    NEE,
    NWW,
    SEE,
    SWW,
}

impl Direction {
    pub fn all() -> Vec<Direction> {
        vec![
            Direction::N,
            Direction::NNE,
            Direction::NE,
            Direction::NEE,
            Direction::E,
            Direction::SEE,
            Direction::SE,
            Direction::SSE,
            Direction::S,
            Direction::SSW,
            Direction::SW,
            Direction::SWW,
            Direction::W,
            Direction::NWW,
            Direction::NW,
            Direction::NNW,
        ]
    }

    pub fn delta(&self) -> Point {
        match self {
            Direction::N => Point::new(0, -1),
            Direction::E => Point::new(1, 0),
            Direction::W => Point::new(-1, 0),
            Direction::S => Point::new(0, 1),
            Direction::NE => Point::new(1, -1),
            Direction::NW => Point::new(-1, -1),
            Direction::SE => Point::new(1, 1),
            Direction::SW => Point::new(-1, 1),
            Direction::NNE => Point::new(1, -2),
            Direction::NNW => Point::new(-1, -2),
            Direction::SSE => Point::new(1, 2),
            Direction::SSW => Point::new(-1, 2),
            Direction::NEE => Point::new(2, -1),
            Direction::NWW => Point::new(-2, -1),
            Direction::SEE => Point::new(2, 1),
            Direction::SWW => Point::new(-2, 1),
        }
    }
}
