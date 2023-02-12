use crate::domain::chessboard::*;
use crate::domain::game::move_rules::*;
use crate::domain::pieces::*;

mod analysis;
mod move_rules;

#[derive(Clone, Debug)]
pub struct ChessGame {
    pub board: Chessboard,
    pub pieces: Vec<Piece>,
    executed_moves: Vec<ExecutedMove>,
}

impl ChessGame {
    /** Create a new Chess Game with initial piece positions. */
    pub fn new() -> Self {
        Self {
            board: Chessboard::new(),
            pieces: create_start_positions(),
            executed_moves: vec![],
        }
    }

    /** The optional piece at a given coordinate. Values range from 0 to 7. */
    pub fn piece_at(&self, coord: (i8, i8)) -> Option<&Piece> {
        self.pieces
            .iter()
            .find(|piece| piece.square.x() == coord.0 as i8 && piece.square.y() == coord.1 as i8)
    }

    /** Execute a given move in this game. No checks are made whether this is an allowed move. */
    pub fn execute_move(&mut self, chosen_move: &Move) {
        if chosen_move.piece.square == chosen_move.target {
            return;
        }
        let mut capture = false;
        if let Some(target_piece) = self.piece_at(chosen_move.target.position()) {
            CapturingMove::new(chosen_move.piece.clone(), target_piece.clone()).execute(self);
            capture = true;
        } else if let Some(rook) = self.castling_rook(chosen_move) {
            CastlingMove::new(chosen_move.piece, *rook).execute(self)
        } else {
            Move::new(chosen_move.piece.clone(), chosen_move.target).execute(self);
        }
        let executed_move = ExecutedMove::new_from(chosen_move, capture, self.is_chess());
        self.executed_moves.push(executed_move);
    }

    pub fn executed_moves(&self) -> Vec<ExecutedMove> {
        self.executed_moves.clone()
    }

    /** Whether a given piece has already moved in this game. */
    pub fn has_already_moved(&self, piece: &Piece) -> bool {
        self.executed_moves()
            .iter()
            .any(|executed_move| executed_move.start_square == piece.start_square())
    }

    /** A collection of all [Move]s which denote a chess in the current game. */
    pub fn chess(&self) -> Vec<Move> {
        let mut result = Vec::new();
        self.pieces
            .iter()
            .filter(|p| p.piece_type == PieceType::King)
            .for_each(|king| {
                self.square_challengers(&king.square)
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

    /** Whether any of the kings of this game is currently in chess. */
    pub fn is_chess(&self) -> bool {
        [PieceColor::White, PieceColor::Black]
            .iter()
            .any(|color| self.is_chess_color(*color))
    }

    /** The optional castling rook for a given move. Only present for Kings. */
    pub fn castling_rook(&self, a_move: &Move) -> Option<&Piece> {
        if a_move.piece.piece_type != PieceType::King {
            return None;
        }
        let king_pos = a_move.piece.square.position();
        let target_pos = a_move.target.position();
        let king_delta = (king_pos.0 - target_pos.0, king_pos.1 - target_pos.1);
        if king_delta.0.abs() < 2 {
            return None;
        }
        return match a_move.target.file().as_str() {
            "g" => self.piece_at((7, a_move.target.y())),
            _ => self.piece_at((0, a_move.target.y())),
        };
    }
}

fn create_start_positions() -> Vec<Piece> {
    let mut result = Vec::new();
    for color in [PieceColor::White, PieceColor::Black] {
        result.push(create_king_start(color));
        result.push(create_queen_start(color));
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

fn create_queen_start(color: PieceColor) -> Piece {
    let row = match color {
        PieceColor::White => '1',
        PieceColor::Black => '8',
    };
    Piece::new(PieceType::Queen, color, BoardSquare::new(row, 'd'))
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

/** A single potential Chess game move of a piece onto an empty target field. */
#[derive(Debug, Clone)]
pub struct Move {
    pub piece: Piece,
    pub target: BoardSquare,
}

impl Move {
    pub fn new(piece: Piece, target: BoardSquare) -> Self {
        Self { piece, target }
    }
}

#[derive(Clone, Debug)]
pub struct ExecutedMove {
    pub piece: Piece,
    pub start_square: BoardSquare,
    pub target_square: BoardSquare,
    pub is_capture: bool,
    pub is_chess: bool,
}

impl ExecutedMove {
    fn new(
        piece: Piece,
        start_square: BoardSquare,
        target_square: BoardSquare,
        is_capture: bool,
        is_chess: bool,
    ) -> Self {
        Self {
            piece,
            start_square,
            target_square,
            is_capture,
            is_chess,
        }
    }
    fn new_from(source_move: &Move, is_capture: bool, is_chess: bool) -> Self {
        Self::new(
            source_move.piece,
            source_move.piece.square,
            source_move.target,
            is_capture,
            is_chess,
        )
    }
    /** Whether this executed move represents a castling. */
    pub fn is_castling(&self) -> bool {
        is_castling_move(&self.piece, &self.start_square, &self.target_square)
    }

    /** This move's coordinate notation string. Includes, chess, capture, and castling. */
    pub fn coord_notation(&self) -> String {
        let start = self.start_square;
        let target = self.target_square;
        if self.is_castling() {
            return match target.file().as_str() {
                "g" => "0-0",
                _ => "0-0-0",
            }
            .to_string();
        }
        format!(
            "{}{}{}{}{}{}",
            start.file().to_uppercase(),
            start.rank(),
            if self.is_capture { 'x' } else { '-' },
            target.file().to_uppercase(),
            target.rank(),
            match self.is_chess {
                true => "+",
                false => "",
            },
        )
    }
}