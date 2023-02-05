use bracket_lib::prelude::Point;

use crate::chessboard::BoardSquare;
use crate::move_rules::Direction;

#[derive(Clone, Debug, PartialEq, Copy)]
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

#[derive(Clone, Debug, PartialEq, Copy)]
pub struct Piece {
    pub piece_type: PieceType,
    pub color: PieceColor,
    pub square: BoardSquare,
}

impl Piece {
    pub(crate) fn new(piece_type: PieceType, color: PieceColor, position: BoardSquare) -> Self {
        Self {
            piece_type,
            color,
            square: position,
        }
    }
}

#[derive(Eq, PartialEq, Copy, Clone, Debug, Hash)]
pub struct PieceDelta {
    pub delta: Point,
    pub max_distance: i8,
}

impl PieceDelta {
    pub fn new(delta: Point, max_distance: i8) -> Self {
        Self {
            delta,
            max_distance,
        }
    }
}

pub fn piece_deltas(piece: &Piece) -> Vec<PieceDelta> {
    return match piece.piece_type {
        PieceType::King => king_move_deltas(),
        PieceType::Rook => rook_move_deltas(),
        PieceType::Bishop => bishop_move_deltas(),
        PieceType::Knight => knight_move_deltas(),
        _ => Vec::new(),
    };
}

fn king_move_deltas() -> Vec<PieceDelta> {
    Direction::adjacent()
        .iter()
        .map(|dir| PieceDelta::new(dir.delta(), 1))
        .collect()
}

fn rook_move_deltas() -> Vec<PieceDelta> {
    Direction::rank_or_file()
        .iter()
        .map(|dir| PieceDelta::new(dir.delta(), 7))
        .collect()
}

fn bishop_move_deltas() -> Vec<PieceDelta> {
    Direction::diagonally()
        .iter()
        .map(|dir| PieceDelta::new(dir.delta(), 7))
        .collect()
}

fn knight_move_deltas() -> Vec<PieceDelta> {
    [
        Direction::NNW,
        Direction::NNE,
        Direction::NEE,
        Direction::SEE,
        Direction::SSE,
        Direction::SSW,
        Direction::SWW,
        Direction::NWW,
    ]
    .iter()
    .map(|dir| PieceDelta::new(dir.delta(), 1))
    .collect()
}
