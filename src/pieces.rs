use crate::chessboard::BoardSquare;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum PieceColor {
    White,
    Black,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Piece {
    pub name: String,
    pub glyph: char,
    pub color: PieceColor,
    pub position: BoardSquare,
}

impl Piece {
    pub(crate) fn new(name: String, glyph: char, color: PieceColor, position: BoardSquare) -> Self {
        Self {
            name,
            glyph,
            color,
            position,
        }
    }
}
