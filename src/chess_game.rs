use bracket_lib::geometry::Point;

use crate::chessboard::*;
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
            .find(|p| p.position.x() == point.x as i8 && p.position.y() == point.y as i8)
    }
}

fn create_start_positions() -> Vec<Piece> {
    let mut result = Vec::new();
    for color in [PieceColor::White, PieceColor::Black] {
        result.push(create_king_start(color));
    }
    return result;
}

fn create_king_start(color: PieceColor) -> Piece {
    Piece::new(
        "King".to_string(),
        'K',
        color,
        BoardSquare {
            row: match color {
                PieceColor::White => '1',
                PieceColor::Black => '8',
            },
            column: 'e',
        },
    )
}
