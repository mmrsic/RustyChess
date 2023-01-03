use bracket_lib::prelude::Point;

use crate::chessboard::BoardSquare;

#[derive(Clone, Debug, PartialEq)]
pub enum PieceType {
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum PieceColor {
    White,
    Black,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Piece {
    pub piece_type: PieceType,
    pub color: PieceColor,
    pub position: BoardSquare,
}

impl Piece {
    pub(crate) fn new(piece_type: PieceType, color: PieceColor, position: BoardSquare) -> Self {
        Self {
            piece_type,
            color,
            position,
        }
    }
}

pub fn piece_deltas(piece: &Piece) -> Vec<Point> {
    king_move_deltas()
}

fn king_move_deltas() -> Vec<Point> {
    let mut deltas = Vec::new();
    for delta_x in -1..=1 {
        for delta_y in -1..=1 {
            if delta_x != 0 || delta_y != 0 {
                deltas.push(Point::new(delta_x, delta_y));
            }
        }
    }
    deltas
}
