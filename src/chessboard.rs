use std::fmt::{Display, Formatter};

use bracket_lib::prelude::Point;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum SquareColor {
    White,
    Black,
}

/** A single chessboard consisting of squares, organized in rows and columns. */
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
    pub fn square_at(&self, coord: Point) -> Option<&BoardSquare> {
        self.squares
            .iter()
            .find(|s| s.x() == (coord.x as i8) && s.y() == (coord.y as i8))
    }
    /** Get a square positioned relatively to a given square. */
    pub fn get_square_relative(&self, square: BoardSquare, delta: &Point) -> Option<&BoardSquare> {
        return self.square_at(square.position() + *delta);
    }
}

/** A single chessboard square, assigned to a row/column combination. */
#[derive(Copy, Clone, Debug, Eq, Hash)]
pub struct BoardSquare {
    /** The row of this square, i.e. one of the values '1' through '8'. */
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
        write!(f, "{}{}", self.column, self.row)
    }
}

impl BoardSquare {
    pub(crate) fn new(row: char, column: char) -> Self {
        Self { row, column }
    }

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
    pub fn position(&self) -> Point {
        return Point::new(self.x(), self.y());
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
