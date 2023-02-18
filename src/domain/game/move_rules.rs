use crate::domain::game::*;

/** A trait which denotes a single move within a game of Chess. */
pub trait ChessGameMove {
    /** Execute the [ChessGameMove]. */
    fn execute(&self, game: &mut ChessGame);
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

pub struct EnPassantMove {
    pub attacker: Piece,
    pub victim_move: ExecutedMove,
    pub target: BoardSquare,
}

impl EnPassantMove {
    pub fn new(attacker: &Piece, victim_move: &ExecutedMove, target: &BoardSquare) -> Self {
        Self {
            attacker: *attacker,
            victim_move: victim_move.clone(),
            target: *target,
        }
    }
}

impl ChessGameMove for EnPassantMove {
    fn execute(&self, game: &mut ChessGame) {
        game.pieces
            .retain(|game_piece| game_piece.square != self.victim_move.target_square);
        Move::new(self.attacker.clone(), self.target).execute(game);
    }
}

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
            true => (2, 0),
            false => (-2, 0),
        };
        let king_square = self.king.square;
        let king_target = game
            .board
            .square_relative(king_square, king_delta)
            .expect(format!("King's castling move square is missing: {:?}", king_square).as_str());
        Move::new(self.king, *king_target).execute(game);
        let rook_delta = match is_kingside {
            true => (-2 as i8, 0 as i8),
            false => (3 as i8, 0 as i8),
        };
        let rook_square = self.rook.square;
        let rook_target = game
            .board
            .square_relative(rook_square, rook_delta)
            .expect(format!("Rook's castling move square is missing: {:?}", rook_square).as_str());
        Move::new(self.rook, *rook_target).execute(game);
    }
}

/** Whether a given piece (including start square) and a given move target square denote a castling
move. This is the case if the piece is a King and the square's files/x-positions differ with more
than one.*/
pub fn is_castling_move(piece: &Piece, start: &BoardSquare, target: &BoardSquare) -> bool {
    piece.piece_type == PieceType::King && (start.x() - target.x()).abs() > 1
}
