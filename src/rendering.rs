use bracket_lib::prelude::*;

use crate::chessboard::*;
use crate::move_rules::*;
use crate::pieces::*;

pub const TILE_WIDTH: i32 = 64;
pub const TILE_HEIGHT: i32 = 64;

const CONSOLE_BOARD: usize = 0;
const CONSOLE_PIECES: usize = 1;

const FONT_FACTOR: i32 = 4;
const BLOCK_CODE: char = '\u{2588}';
const POSSIBLE_MOVE_CODE: char = '\u{2591}';
const BACKGROUND: (u8, u8, u8) = LIGHT_GREEN;

pub fn create_gui() -> BTerm {
    BTermBuilder::simple(8, 8)
        .unwrap()
        .with_fps_cap(25.0)
        .with_title("C H E S S")
        .with_tile_dimensions(TILE_WIDTH, TILE_HEIGHT)
        .with_simple_console_no_bg(8, 8, "terminal8x8.png")
        .with_simple_console_no_bg(8 * FONT_FACTOR, 8 * FONT_FACTOR, "terminal8x8.png")
        .with_advanced_input(true)
        .build()
        .unwrap()
}

/** Render a given [Chessboard] onto a given [BTerm]. */
pub fn render_board(board: &Chessboard, ctx: &mut BTerm) {
    ctx.set_active_console(CONSOLE_BOARD);
    ctx.cls();
    for square in board.squares() {
        ctx.set(
            square.x(),
            square.y(),
            to_square_ui_color(square.color()),
            BACKGROUND,
            to_cp437(BLOCK_CODE),
        );
    }
}

/** Render a collection of pieces onto a given [BTerm]. */
pub fn render_pieces(pieces: &Vec<Piece>, ctx: &mut BTerm) {
    ctx.set_active_console(CONSOLE_PIECES);
    ctx.cls();
    for piece in pieces {
        render_piece(&piece, ctx);
    }
}

/** Render a single given [Piece] onto a given [BTerm]. */
pub fn render_piece(piece: &Piece, ctx: &mut BTerm) {
    ctx.set_active_console(CONSOLE_PIECES);
    ctx.set(
        piece.position.x(),
        piece.position.y(),
        to_piece_ui_color(piece.color),
        to_square_ui_color(piece.position.color()),
        to_cp437(match piece.piece_type {
            PieceType::King => 'K',
            PieceType::Queen => 'q',
            PieceType::Rook => 'r',
            PieceType::Bishop => 'b',
            PieceType::Knight => 'k',
            PieceType::Pawn => 'p',
        }),
    );
}

fn to_square_ui_color(piece_color: SquareColor) -> (u8, u8, u8) {
    return match piece_color {
        SquareColor::White => LIGHT_GRAY,
        SquareColor::Black => DARK_GRAY,
    };
}

fn to_piece_ui_color(piece_color: PieceColor) -> (u8, u8, u8) {
    return match piece_color {
        PieceColor::White => WHITE,
        PieceColor::Black => BLACK,
    };
}

pub fn render_possible_moves(possible_moves: Vec<Move>, ctx: &mut BTerm) {
    ctx.set_active_console(CONSOLE_BOARD);
    possible_moves.iter().for_each(|possible_move| {
        let target_square = possible_move.target;
        ctx.set(
            target_square.x(),
            target_square.y(),
            LIGHT_GREEN,
            to_square_ui_color(possible_move.target.color()),
            to_cp437(POSSIBLE_MOVE_CODE),
        );
    });
}
