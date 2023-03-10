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
    chess_moves: Vec<Move>,
    promotion_pawn: Option<Piece>,
    initial_color: PieceColor,
}

impl ChessGame {
    /** Create a new Chess Game with initial piece positions. */
    pub fn new() -> Self {
        Self {
            board: Chessboard::new(),
            pieces: create_start_positions(),
            executed_moves: vec![],
            chess_moves: vec![],
            promotion_pawn: None,
            initial_color: PieceColor::White,
        }
    }

    /** The optional piece at a given coordinate. Values range from 0 to 7. */
    pub fn piece_at(&self, coord: (i8, i8)) -> Option<&Piece> {
        self.pieces
            .iter()
            .find(|piece| piece.square.x() == coord.0 as i8 && piece.square.y() == coord.1 as i8)
    }

    /** The (only) pawn which must be promoted before the next move can be made.
    See: [ChessGame::exchange_promotion_pawn]*/
    pub fn promotion_pawn(&self) -> Option<Piece> {
        self.promotion_pawn.clone()
    }

    /** Exchange the current pawn waiting for promotion with a new piece type.
    See: [ChessGame::promotion_pawn]. */
    pub fn exchange_promotion_pawn(&mut self, new_type: PieceType) {
        if let Some(promotion_pawn) = self.promotion_pawn {
            self.replace_piece_type(promotion_pawn, new_type);
            self.promotion_pawn = None;
            self.chess_moves = self.calculate_check();
        }
    }

    fn replace_piece_type(&mut self, old_piece: Piece, new_type: PieceType) {
        self.pieces
            .iter_mut()
            .filter(|game_piece| game_piece.square == old_piece.square)
            .for_each(|game_piece| game_piece.piece_type = new_type);
    }

    /** Execute a given move in this game. No checks are made whether this is an allowed move. */
    pub fn execute_move(&mut self, chosen_move: &Move) {
        if chosen_move.piece.square == chosen_move.target {
            return;
        }
        if let Some(pawn) = self.promotion_pawn {
            println!(
                "Cannot execute move: Pawn must be converted first: {:?}",
                pawn
            );
            return;
        }
        if self.next_move_color() != chosen_move.piece.color {
            return;
        }

        let mut capture = false;
        let en_passant_target = self.en_passant_target();
        if en_passant_target.is_some() && chosen_move.target == *en_passant_target.unwrap() {
            let victim_move = self.executed_moves.last().unwrap();
            EnPassantMove::new(&chosen_move.piece, victim_move, en_passant_target.unwrap())
                .execute(self);
            capture = true;
        } else if let Some(target_piece) = self.piece_at(chosen_move.target.position()) {
            CapturingMove::new(chosen_move.piece.clone(), target_piece.clone()).execute(self);
            capture = true;
        } else if let Some(rook) = self.castling_rook(chosen_move) {
            CastlingMove::new(chosen_move.piece, *rook).execute(self)
        } else {
            Move::new(chosen_move.piece.clone(), chosen_move.target).execute(self);
        }
        let executed_move = ExecutedMove::new_from(chosen_move, capture, self.is_check());
        self.executed_moves.push(executed_move);
        self.chess_moves = self.calculate_check();
        self.promotion_pawn = self.check_promotion_pawn().cloned();
    }

    /** All the moves of this game in the order they were executed. */
    pub fn executed_moves(&self) -> Vec<ExecutedMove> {
        self.executed_moves.clone()
    }

    /** Whether a given piece has already moved in this game. */
    pub fn has_already_moved(&self, piece: &Piece) -> bool {
        self.executed_moves()
            .iter()
            .any(|executed_move| executed_move.start_square == piece.start_square())
    }

    /** The piece color which is allowed to move next. */
    pub fn next_move_color(&self) -> PieceColor {
        if self.executed_moves().len() % 2 == 0 {
            return self.initial_color;
        };
        self.initial_color.opponent()
    }

    /** A collection of all [Move]s which denote a chess in the current game. */
    pub fn chess_moves(&self) -> Vec<Move> {
        self.chess_moves.clone()
    }

    /** Calculate a collection of all [Move]s which denote a chess in the current game. */
    fn calculate_check(&self) -> Vec<Move> {
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

    /** Whether the king of a given piece color is currently in check. */
    pub fn is_check_color(&self, color: PieceColor) -> bool {
        self.calculate_check()
            .iter()
            .any(|chess_move| chess_move.piece.color != color)
    }

    /** Whether any of the kings of this game is currently in check. */
    pub fn is_check(&self) -> bool {
        [PieceColor::White, PieceColor::Black]
            .iter()
            .any(|color| self.is_check_color(*color))
    }

    /** The optional castling rook for a given move. Only present for Kings. */
    fn castling_rook(&self, a_move: &Move) -> Option<&Piece> {
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
    /** En passant target for a given executed move. */
    fn en_passant_target(&self) -> Option<&BoardSquare> {
        let all_moves = self.executed_moves();
        let last_move = all_moves.last();
        if last_move.is_none() {
            return None;
        }
        let mov = last_move.unwrap();
        if mov.piece.piece_type != PieceType::Pawn {
            return None;
        }
        let y_delta = mov.delta().1;
        if y_delta.abs() != 2 {
            return None;
        }
        self.board
            .square_relative(mov.start_square, (mov.delta().0, y_delta.signum()))
    }

    /** Optional Pawn that must be promoted before game may continue. */
    fn check_promotion_pawn(&self) -> Option<&Piece> {
        self.pieces
            .iter()
            .filter(|piece| piece.piece_type == PieceType::Pawn)
            .find(|pawn| ["1".to_string(), "8".to_string()].contains(&pawn.square.rank()))
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
            .for_each(|bishop| result.push(bishop.clone()));
        create_pawns_start(color)
            .iter()
            .for_each(|pawn| result.push(pawn.clone()));
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

fn create_pawns_start(color: PieceColor) -> Vec<Piece> {
    let row = match color {
        PieceColor::White => '2',
        PieceColor::Black => '7',
    };
    let mut result = Vec::new();
    ('a'..='h').for_each(|column| {
        let square = BoardSquare::new(row, column);
        result.push(Piece::new(PieceType::Pawn, color, square))
    });
    result
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
    pub fn delta(&self) -> (i8, i8) {
        let start_pos = self.start_square.position();
        let target_pos = self.target_square.position();
        (target_pos.0 - start_pos.0, target_pos.1 - start_pos.1)
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
