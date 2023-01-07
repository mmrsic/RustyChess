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
            .filter(|square| {
                let other_piece = self.piece_at(square.position());
                None == other_piece || other_piece.unwrap().color != piece.color
            })
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
