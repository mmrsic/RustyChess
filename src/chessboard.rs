use std::fmt::{Display, Formatter};

use bracket_lib::geometry::Point;

use crate::pieces::PieceColor;

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
    pub fn square_at(&self, coord: Point) -> Option<&BoardSquare> {
        self.squares
            .iter()
            .find(|s| s.x() == (coord.x as i8) && s.y() == (coord.y as i8))
    }
    pub fn get_square_relative(&self, original: BoardSquare, delta: Point) -> Option<&BoardSquare> {
        self.square_at(Point::new(
            original.x() + delta.x as i8,
            original.y() + delta.y as i8,
        ))
    }
}

/** A single chessboard square, assigned to a row/column combination. */
#[derive(Copy, Clone, Debug)]
pub struct BoardSquare {
    /** The row of this square, i.e. one of the values '1' through '8'. */
    pub row: char,
    pub column: char,
}

impl Display for BoardSquare {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.column, self.row)
    }
}

impl BoardSquare {
    fn new(row: char, column: char) -> Self {
        Self { row, column }
    }

    /** The rows are called ranks. */
    pub(crate) fn rank(&self) -> String {
        return self.row.to_string();
    }
    /** The columns are called files. */
    pub(crate) fn file(&self) -> String {
        return self.column.to_string();
    }
    /** The x position of this square. This depends on the column, i.e. column 'a' is x=0. */
    pub(crate) fn x(&self) -> i8 {
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
    pub(crate) fn y(&self) -> i8 {
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

    pub(crate) fn color(&self) -> PieceColor {
        return match self.row {
            '8' | '6' | '4' | '2' => match self.column {
                'a' | 'c' | 'e' | 'g' => PieceColor::White,
                _ => PieceColor::Black,
            },
            _ => match self.column {
                'a' | 'c' | 'e' | 'g' => PieceColor::Black,
                _ => PieceColor::White,
            },
        };
    }
}
