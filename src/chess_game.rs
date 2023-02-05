use bracket_lib::geometry::Point;

use crate::chessboard::*;
use crate::move_rules::*;
use crate::pieces::*;

#[derive(Clone, Debug)]
pub struct ChessGame {
    pub board: Chessboard,
    pub pieces: Vec<Piece>,
}

impl ChessGame {
    /** Create a new Chess Game with initial piece positions. */
    pub fn new() -> Self {
        Self {
            board: Chessboard::new(),
            pieces: create_start_positions(),
        }
    }

    /** The optional piece at a given coordinate. */
    pub fn piece_at(&self, coord: Point) -> Option<&Piece> {
        self.pieces
            .iter()
            .find(|piece| piece.square.x() == coord.x as i8 && piece.square.y() == coord.y as i8)
    }

    /** Execute a given move in this game. No checks are made whether this is an allowed move. */
    pub fn execute_move(&mut self, chosen_move: &Move) {
        if let Some(target_piece) = self.piece_at(chosen_move.target.position()) {
            CapturingMove::new(chosen_move.piece.clone(), target_piece.clone()).execute(self);
        } else {
            Move::new(chosen_move.piece.clone(), chosen_move.target).execute(self);
        }
    }

    /** A collection of all [Move]s which denote a chess in the current game. */
    pub fn chess(&self) -> Vec<Move> {
        let mut result = Vec::new();
        self.pieces
            .iter()
            .filter(|p| p.piece_type == PieceType::King)
            .for_each(|king| {
                self.square_contesters(&king.square)
                    .iter()
                    .filter(|candidate| candidate.color != king.color)
                    .for_each(|chess_piece| result.push(Move::new(*chess_piece, king.square)))
            });
        result
    }

    /** Whether the king of a given piece color is currently in chess. */
    pub fn is_chess_color(&self, color: PieceColor) -> bool {
        self.chess()
            .iter()
            .any(|chess_move| chess_move.piece.color != color)
    }
}

fn create_start_positions() -> Vec<Piece> {
    let mut result = Vec::new();
    for color in [PieceColor::White, PieceColor::Black] {
        result.push(create_king_start(color));
        create_knights_start(color)
            .iter()
            .for_each(|knight| result.push(knight.clone()));
        create_rooks_start(color)
            .iter()
            .for_each(|rook| result.push(rook.clone()));
        create_bishops_start(color)
            .iter()
            .for_each(|bishop| result.push(bishop.clone()))
    }
    return result;
}

fn create_king_start(color: PieceColor) -> Piece {
    let row = match color {
        PieceColor::White => '1',
        PieceColor::Black => '8',
    };
    Piece::new(PieceType::King, color, BoardSquare::new(row, 'e'))
}

fn create_knights_start(color: PieceColor) -> Vec<Piece> {
    let row = match color {
        PieceColor::White => '1',
        PieceColor::Black => '8',
    };
    vec![
        Piece::new(PieceType::Knight, color, BoardSquare::new(row, 'b')),
        Piece::new(PieceType::Knight, color, BoardSquare::new(row, 'g')),
    ]
}

fn create_rooks_start(color: PieceColor) -> Vec<Piece> {
    let row = match color {
        PieceColor::White => '1',
        PieceColor::Black => '8',
    };
    vec![
        Piece::new(PieceType::Rook, color, BoardSquare::new(row, 'a')),
        Piece::new(PieceType::Rook, color, BoardSquare::new(row, 'h')),
    ]
}

fn create_bishops_start(color: PieceColor) -> Vec<Piece> {
    let row = match color {
        PieceColor::White => '1',
        PieceColor::Black => '8',
    };
    vec![
        Piece::new(PieceType::Bishop, color, BoardSquare::new(row, 'c')),
        Piece::new(PieceType::Bishop, color, BoardSquare::new(row, 'f')),
    ]
}
