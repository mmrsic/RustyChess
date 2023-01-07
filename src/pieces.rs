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
    return match piece.piece_type {
        PieceType::King => king_move_deltas(),
        PieceType::Knight => knight_move_deltas(),
        _ => Vec::new(),
    };
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

fn knight_move_deltas() -> Vec<Point> {
    vec![
        Point::new(-1, -2),
        Point::new(1, -2),
        Point::new(2, -1),
        Point::new(2, 1),
        Point::new(1, 2),
        Point::new(-1, 2),
        Point::new(-2, 1),
        Point::new(-2, -1),
    ]
}
