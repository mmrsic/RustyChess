use bracket_lib::prelude::Point;

use crate::chess_game::ChessGame;
use crate::chessboard::BoardSquare;
use crate::pieces::{Piece, PieceType};

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
    }
}

impl ChessGameMove for CapturingMove {
    fn execute(&self, game: &mut ChessGame) {
        game.pieces
            .retain(|game_piece| game_piece.square != self.victim.square);
        Move::new(self.attacker.clone(), self.victim.square).execute(game);
    }
}

/*

pub struct EnPassantMove {
    pub attacker: Piece,
    pub victim: Piece,
    pub target: BoardSquare,
}

 */

pub struct CastlingMove {
    pub king: Piece,
    pub rook: Piece,
}

impl CastlingMove {
    pub fn new(king: Piece, rook: Piece) -> Self {
        Self { king, rook }
    }
    /** Whether this castling move is taking place at the King's side. */
    pub fn is_kingside(&self) -> bool {
        self.king.square.x() < self.rook.square.x()
    }
}

impl ChessGameMove for CastlingMove {
    fn execute(&self, game: &mut ChessGame) {
        let is_kingside = self.is_kingside();
        let king_delta = match is_kingside {
            true => Point::new(2, 0),
            false => Point::new(-2, 0),
        };
        let king_square = self.king.square;
        let king_target = game
            .board
            .get_square_relative(king_square, &king_delta)
            .expect(format!("King's castling move square is missing: {:?}", king_square).as_str());
        Move::new(self.king, *king_target).execute(game);
        let rook_delta = match is_kingside {
            true => Point::new(-2, 0),
            false => Point::new(3, 0),
        };
        let rook_square = self.rook.square;
        let rook_target = game
            .board
            .get_square_relative(rook_square, &rook_delta)
            .expect(format!("Rook's castling move square is missing: {:?}", rook_square).as_str());
        Move::new(self.rook, *rook_target).execute(game);
    }
}

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
    pub fn rank_or_file() -> Vec<Direction> {
        vec![Direction::N, Direction::E, Direction::S, Direction::W]
    }
    pub fn diagonally() -> Vec<Direction> {
        vec![Direction::NE, Direction::SE, Direction::SW, Direction::NW]
    }
    pub fn adjacent() -> Vec<Direction> {
        let mut result = Direction::rank_or_file();
        let mut additional = Direction::diagonally();
        result.append(&mut additional);
        result
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

/** Whether a given piece (including start square) and a given move target square denote a castling
move. This is the case if the piece is a King and the square's files/x-positions differ with more
than one.*/
pub fn is_castling_move(piece: &Piece, start: &BoardSquare, target: &BoardSquare) -> bool {
    piece.piece_type == PieceType::King && (start.x() - target.x()).abs() > 1
}
