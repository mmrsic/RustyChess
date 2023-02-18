use crate::domain::chessboard::BoardSquare;

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
    start_square: BoardSquare,
}

impl Piece {
    pub(crate) fn new(piece_type: PieceType, color: PieceColor, position: BoardSquare) -> Self {
        Self {
            piece_type,
            color,
            square: position,
            start_square: position,
        }
    }
    /** The square a given piece started the game. */
    pub fn start_square(&self) -> BoardSquare {
        self.start_square
    }
    /** Whether this piece is at it's start square. */
    pub fn is_at_start(&self) -> bool {
        self.start_square == self.square
    }
}

#[derive(Eq, PartialEq, Copy, Clone, Debug, Hash)]
pub struct PieceDelta {
    pub delta: (i8, i8),
    pub max_distance: i8,
    pub capture_policy: CapturePolicy,
}

impl PieceDelta {
    pub fn new(delta: (i8, i8), max_distance: i8) -> Self {
        Self {
            delta,
            max_distance,
            capture_policy: CapturePolicy::Allowed,
        }
    }
    pub fn may_capture(&self) -> bool {
        [CapturePolicy::Mandatory, CapturePolicy::Allowed].contains(&self.capture_policy)
    }
}

pub fn piece_deltas(piece: &Piece) -> Vec<PieceDelta> {
    return match piece.piece_type {
        PieceType::King => king_move_deltas(),
        PieceType::Queen => queen_move_deltas(),
        PieceType::Rook => rook_move_deltas(),
        PieceType::Bishop => bishop_move_deltas(),
        PieceType::Knight => knight_move_deltas(),
        PieceType::Pawn => pawn_move_deltas(piece),
    };
}

fn king_move_deltas() -> Vec<PieceDelta> {
    Direction::adjacent()
        .iter()
        .map(|dir| PieceDelta::new(dir.delta(), 1))
        .collect()
}

fn queen_move_deltas() -> Vec<PieceDelta> {
    Direction::adjacent()
        .iter()
        .map(|dir| PieceDelta::new(dir.delta(), 7))
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

fn pawn_move_deltas(pawn: &Piece) -> Vec<PieceDelta> {
    let mut result = Vec::new();
    let dir = match pawn.color {
        PieceColor::White => Direction::N,
        PieceColor::Black => Direction::S,
    };
    let max_dist = if pawn.is_at_start() { 2 } else { 1 };
    let mut default_delta = PieceDelta::new(dir.delta(), max_dist);
    default_delta.capture_policy = CapturePolicy::Forbidden;
    result.push(default_delta);
    if pawn.square.column() != "a" {
        let mut left_capture_delta = default_delta.clone();
        left_capture_delta.delta = (left_capture_delta.delta.0 - 1, left_capture_delta.delta.1);
        left_capture_delta.capture_policy = CapturePolicy::Mandatory;
        left_capture_delta.max_distance = 1;
        result.push(left_capture_delta);
    }
    if pawn.square.column() != "h" {
        let mut right_capture_delta = default_delta.clone();
        right_capture_delta.delta = (right_capture_delta.delta.0 + 1, right_capture_delta.delta.1);
        right_capture_delta.capture_policy = CapturePolicy::Mandatory;
        right_capture_delta.max_distance = 1;
        result.push(right_capture_delta);
    }
    result
}

#[derive(Eq, PartialEq, Hash, Clone, Debug)]
pub enum Direction {
    N,
    E,
    W,
    S,
    NE,
    NW,
    SE,
    SW,
    NNE,
    NNW,
    SSE,
    SSW,
    NEE,
    NWW,
    SEE,
    SWW,
}

impl Direction {
    pub fn all() -> Vec<Direction> {
        vec![
            Direction::N,
            Direction::NNE,
            Direction::NE,
            Direction::NEE,
            Direction::E,
            Direction::SEE,
            Direction::SE,
            Direction::SSE,
            Direction::S,
            Direction::SSW,
            Direction::SW,
            Direction::SWW,
            Direction::W,
            Direction::NWW,
            Direction::NW,
            Direction::NNW,
        ]
    }
    pub fn rank_or_file() -> Vec<Direction> {
        vec![Direction::N, Direction::E, Direction::S, Direction::W]
    }
    pub fn diagonally() -> Vec<Direction> {
        vec![Direction::NE, Direction::SE, Direction::SW, Direction::NW]
    }
    pub fn adjacent() -> Vec<Direction> {
        let mut result = Direction::rank_or_file();
        let mut additional = Direction::diagonally();
        result.append(&mut additional);
        result
    }

    pub fn delta(&self) -> (i8, i8) {
        match self {
            Direction::N => (0, -1),
            Direction::E => (1, 0),
            Direction::W => (-1, 0),
            Direction::S => (0, 1),
            Direction::NE => (1, -1),
            Direction::NW => (-1, -1),
            Direction::SE => (1, 1),
            Direction::SW => (-1, 1),
            Direction::NNE => (1, -2),
            Direction::NNW => (-1, -2),
            Direction::SSE => (1, 2),
            Direction::SSW => (-1, 2),
            Direction::NEE => (2, -1),
            Direction::NWW => (-2, -1),
            Direction::SEE => (2, 1),
            Direction::SWW => (-2, 1),
        }
    }
}

#[derive(Eq, PartialEq, Copy, Clone, Debug, Hash)]
pub enum CapturePolicy {
    Mandatory,
    Allowed,
    Forbidden,
}
