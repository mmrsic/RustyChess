use bracket_lib::prelude::*;

use crate::chess_game::ChessGame;
use crate::chessboard::*;
use crate::move_rules::*;
use crate::pieces::*;

pub const TILE_WIDTH: i32 = 64;
pub const TILE_HEIGHT: i32 = 64;

const CONSOLE_BOARD: usize = 0;
const CONSOLE_PIECES: usize = 1;
const CONSOLE_TEXTS: usize = 2;

const GRAPHICS_WIDTH: i32 = 12;
const GRAPHICS_HEIGHT: i32 = 8;
const FONT_WIDTH_FACTOR: i32 = 8;
const FONT_HEIGHT_FACTOR: i32 = 4;
const TEXT_WIDTH: i32 = GRAPHICS_WIDTH * FONT_WIDTH_FACTOR;
const TEXT_HEIGHT: i32 = GRAPHICS_HEIGHT * FONT_HEIGHT_FACTOR;
const TEXT_LEFT_START: i32 = 8 * FONT_WIDTH_FACTOR + 1;

const BLOCK_CODE: char = '\u{2588}';
const POSSIBLE_MOVE_CODE: char = '\u{2591}';
const CHESS_CODE: char = '\u{2591}';
const BACKGROUND: (u8, u8, u8) = LIGHT_GREEN;

const TEXT_FILE: &'static str = "terminal8x8.png";
const CHESS_PIECES_FILE: &'static str = "chesspieces.png";
const KING_OFFSET: i32 = 0;
const QUEEN_OFFSET: i32 = KING_OFFSET + 1;
const ROOK_OFFSET: i32 = QUEEN_OFFSET + 1;
const BISHOP_OFFSET: i32 = ROOK_OFFSET + 1;
const KNIGHT_OFFSET: i32 = BISHOP_OFFSET + 1;
const PAWN_OFFSET: i32 = KNIGHT_OFFSET + 1;

pub fn create_gui() -> BTerm {
    BTermBuilder::simple(GRAPHICS_WIDTH, GRAPHICS_HEIGHT)
        .unwrap()
        .with_resource_path("resources")
        .with_font("chesspieces.png", TILE_WIDTH, TILE_HEIGHT)
        .with_fps_cap(25.0)
        .with_title("C H E S S")
        .with_tile_dimensions(TILE_WIDTH, TILE_HEIGHT)
        .with_simple_console_no_bg(GRAPHICS_WIDTH, GRAPHICS_HEIGHT, CHESS_PIECES_FILE)
        .with_simple_console_no_bg(TEXT_WIDTH, TEXT_HEIGHT, TEXT_FILE)
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
        piece.square.x(),
        piece.square.y(),
        to_piece_ui_color(piece.color),
        to_square_ui_color(piece.square.color()),
        match piece.piece_type {
            PieceType::King => KING_OFFSET,
            PieceType::Queen => QUEEN_OFFSET,
            PieceType::Rook => ROOK_OFFSET,
            PieceType::Bishop => BISHOP_OFFSET,
            PieceType::Knight => KNIGHT_OFFSET,
            PieceType::Pawn => PAWN_OFFSET,
        },
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
            to_square_ui_color(target_square.color()),
            to_cp437(POSSIBLE_MOVE_CODE),
        );
    });
}

pub fn render_chess(game: &ChessGame, ctx: &mut BTerm) {
    ctx.set_active_console(CONSOLE_TEXTS);
    ctx.cls();
    let chess_moves = game.chess();
    if !chess_moves.is_empty() {
        ctx.set_active_console(CONSOLE_BOARD);
        let chess_square = chess_moves.iter().nth(0).unwrap().target;
        ctx.set(
            chess_square.x(),
            chess_square.y(),
            RED,
            to_square_ui_color(chess_square.color()),
            to_cp437(CHESS_CODE),
        );
        ctx.set_active_console(CONSOLE_TEXTS);
        ctx.print(TEXT_LEFT_START, 1, "Chess");
    }
    ctx.set_active_console(CONSOLE_PIECES);
}
