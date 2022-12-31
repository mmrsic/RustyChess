use bracket_lib::prelude::{to_cp437, FontCharType};

use crate::chessboard::BoardSquare;

pub fn create_start_positions() -> Vec<Piece> {
    let mut result = Vec::new();
    for color in [PieceColor::White, PieceColor::Black] {
        result.push(king(color));
    }
    return result;
}

#[derive(Copy, Clone, Debug)]
pub enum PieceColor {
    White,
    Black,
}

#[derive(Clone, Debug)]
pub struct Piece {
    pub name: String,
    pub glyph: FontCharType,
    pub color: PieceColor,
    pub position: BoardSquare,
}

impl Piece {
    fn new(name: String, glyph: FontCharType, color: PieceColor, position: BoardSquare) -> Self {
        Self {
            name,
            glyph,
            color,
            position,
        }
    }
}

fn king(color: PieceColor) -> Piece {
    Piece::new(
        "King".to_string(),
        to_cp437('K'),
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
