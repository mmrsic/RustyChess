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
            if game_piece.position == self.piece.position {
                game_piece.position.clone_from(&self.target);
            }
        });
        println!("Executed move: {:?}", self);
    }
}

impl ChessGameMove for CapturingMove {
    fn execute(&self, game: &mut ChessGame) {
        game.pieces
            .retain(|game_piece| game_piece.position != self.victim.position);
        Move::new(self.attacker.clone(), self.victim.position).execute(game);
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
