use std::cmp::{max, min};
use std::fmt::{Display, Formatter};

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum SquareColor {
    White,
    Black,
}

/** A single chessboard consisting of squares, organized in rows and columns. */
#[derive(Clone, Debug)]
pub struct Chessboard {
    squares: Vec<BoardSquare>,
}

impl Chessboard {
    pub fn new() -> Self {
        let mut squares = Vec::new();
        for row in ('1'..='8').rev() {
            for column in 'a'..='h' {
                squares.push(BoardSquare::new(row, column));
            }
        }
        Self { squares }
    }

    /** All the squares of this board in the order from top-left to bottom-right. */
    pub fn squares(&self) -> Vec<BoardSquare> {
        return self.squares.clone();
    }
    /** The optional square at a given coordinate. None if not valid. */
    pub fn square_at(&self, coord: (i8, i8)) -> Option<&BoardSquare> {
        self.squares
            .iter()
            .find(|s| s.x() == coord.0 as i8 && s.y() == coord.1 as i8)
    }
    /** All squares between two given squares of equal ranks or files - including them.
    Empty if the specified squares are neither on the same rank nor on the same file. */
    pub fn all_squares_between(&self, sq1: BoardSquare, sq2: BoardSquare) -> Vec<BoardSquare> {
        let mut result = Vec::new();
        if sq1.y() == sq2.y() {
            for x_i in min(sq1.x(), sq2.x())..=max(sq1.x(), sq2.x()) {
                result.push(*self.square_at((x_i, sq1.y())).unwrap())
            }
        }
        result
    }
    /** Get a square positioned relatively to a given square. */
    pub fn square_relative(&self, square: BoardSquare, delta: (i8, i8)) -> Option<&BoardSquare> {
        return self.square_at((square.position().0 + delta.0, square.position().1 + delta.1));
    }
}

/** A single chessboard square, assigned to a row/column combination. */
#[derive(Copy, Clone, Debug, Eq, Hash)]
pub struct BoardSquare {
    row: char,
    column: char,
}

impl PartialEq<Self> for BoardSquare {
    fn eq(&self, other: &Self) -> bool {
        let result = self.row == other.row && self.column == other.column;
        result
    }
}

impl Display for BoardSquare {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.file(), self.rank())
    }
}

impl BoardSquare {
    pub fn new(row: char, column: char) -> Self {
        Self { row, column }
    }

    /** x position of this square. Depending on the column, e.g. column 'a' is x=0. */
    pub fn x(&self) -> i8 {
        return match self.column {
            'a' => 0,
            'b' => 1,
            'c' => 2,
            'd' => 3,
            'e' => 4,
            'f' => 5,
            'g' => 6,
            _ => 7,
        };
    }
    /** The y position of this square. This depends on the row, i.e. row '8' is y=0. */
    pub fn y(&self) -> i8 {
        return match self.row {
            '8' => 0,
            '7' => 1,
            '6' => 2,
            '5' => 3,
            '4' => 4,
            '3' => 5,
            '2' => 6,
            _ => 7,
        };
    }
    /** The row of this square. */
    pub fn row(&self) -> String {
        return self.row.to_string();
    }
    /** The columns are called files. */
    pub fn column(&self) -> String {
        return self.column.to_string();
    }
    /** The rows are called ranks. */
    pub fn rank(&self) -> String {
        return self.row();
    }
    /** The columns are called files. */
    pub fn file(&self) -> String {
        return self.column();
    }
    pub fn position(&self) -> (i8, i8) {
        return (self.x(), self.y());
    }

    pub fn color(&self) -> SquareColor {
        return match self.row {
            '8' | '6' | '4' | '2' => match self.column {
                'a' | 'c' | 'e' | 'g' => SquareColor::White,
                _ => SquareColor::Black,
            },
            _ => match self.column {
                'a' | 'c' | 'e' | 'g' => SquareColor::Black,
                _ => SquareColor::White,
            },
        };
    }
}
