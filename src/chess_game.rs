use bracket_lib::geometry::Point;

use crate::chessboard::*;
use crate::move_rules::*;
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
            .find(|p| p.square.x() == point.x as i8 && p.square.y() == point.y as i8)
    }

    pub(crate) fn execute_move(&mut self, chosen_move: &Move) {
        if let Some(target_piece) = self.piece_at(chosen_move.target.position()) {
            CapturingMove::new(chosen_move.piece.clone(), target_piece.clone()).execute(self);
        } else {
            Move::new(chosen_move.piece.clone(), chosen_move.target).execute(self);
        }
    }
}

fn create_start_positions() -> Vec<Piece> {
    let mut result = Vec::new();
    for color in [PieceColor::White, PieceColor::Black] {
        result.push(create_king_start(color));
        create_knights_start(color)
            .iter()
            .for_each(|knight| result.push(knight.clone()))
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
