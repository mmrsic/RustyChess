use std::collections::HashSet;

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
            .find(|p| p.position.x() == point.x as i8 && p.position.y() == point.y as i8)
    }

    /** All possible moves a given piece can currently make in this [ChessGame]. */
    pub fn possible_moves(&self, piece: &Piece) -> Vec<Move> {
        piece_deltas(piece)
            .iter()
            .filter_map(|delta| self.board.get_square_relative(piece.position, delta))
            .map(|target| Move {
                piece: piece.clone(),
                target: *target,
            })
            .collect()
    }

    /** All [BoardSquare]s currently threatened by the pieces (applied to a given filter) of this [ChessGame].  */
    pub fn piece_controlled_area<G>(&self, piece_filter: G) -> HashSet<BoardSquare>
    where
        G: FnMut(&&Piece) -> bool,
    {
        let mut result = HashSet::new();
        self.pieces.iter().filter(piece_filter).for_each(|piece| {
            self.possible_moves(piece).iter().for_each(|possible_move| {
                result.insert(possible_move.target);
            })
        });
        return result;
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
    let row = match color {
        PieceColor::White => '1',
        PieceColor::Black => '8',
    };
    Piece::new(PieceType::King, color, BoardSquare::new(row, 'e'))
}
