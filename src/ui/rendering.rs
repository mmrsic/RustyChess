use bracket_lib::prelude::*;

use crate::domain::chessboard::*;
use crate::domain::game::*;
use crate::domain::pieces::*;
use crate::*;

pub const TILE_WIDTH: i32 = 64;
pub const TILE_HEIGHT: i32 = 64;

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

const CHESS_PIECES_FILE: &'static str = "chess_pieces.png";
const TEXT_FILE: &'static str = "terminal8x8.png";

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
        .with_font(CHESS_PIECES_FILE, TILE_WIDTH, TILE_HEIGHT)
        .with_fps_cap(25.0)
        .with_title("C H E S S")
        .with_tile_dimensions(TILE_WIDTH, TILE_HEIGHT)
        .with_sparse_console_no_bg(GRAPHICS_WIDTH, GRAPHICS_HEIGHT, CHESS_PIECES_FILE)
        .with_simple_console_no_bg(TEXT_WIDTH, TEXT_HEIGHT, TEXT_FILE)
        .with_advanced_input(true)
        .build()
        .unwrap()
}

/** Render a given [Chessboard] onto a given [BTerm]. */
pub fn render_board(board: &Chessboard, ctx: &mut BTerm) {
    set_active_console_board(ctx);
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
    set_active_console_pieces(ctx);
    ctx.cls();
    for piece in pieces {
        render_piece(&piece, ctx);
    }
}

/** Render a single given [Piece] onto a given [BTerm]. */
pub fn render_piece(piece: &Piece, ctx: &mut BTerm) {
    set_active_console_pieces(ctx);
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

pub fn set_active_console_pieces(ctx: &mut BTerm) {
    ctx.set_active_console(1);
}

fn set_active_console_board(ctx: &mut BTerm) {
    ctx.set_active_console(0);
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
    set_active_console_board(ctx);
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

pub fn render_game_end_and_check(game: &ChessGame, ctx: &mut BTerm) {
    if game.is_check_mate() {
        render_check_mate(ctx);
    } else if game.is_stalemate() {
        render_stalemate(ctx);
    } else {
        render_check(game, ctx);
    }
}

fn render_check(game: &ChessGame, ctx: &mut BTerm) {
    let chess_moves = game.chess_moves();
    if !chess_moves.is_empty() {
        set_active_console_board(ctx);
        let chess_square = chess_moves.iter().nth(0).unwrap().target;
        ctx.set(
            chess_square.x(),
            chess_square.y(),
            RED,
            to_square_ui_color(chess_square.color()),
            to_cp437(CHESS_CODE),
        );
    }
}

fn render_check_mate(ctx: &mut BTerm) {
    set_active_console_texts(ctx);
    ctx.cls();
    ctx.print(TEXT_LEFT_START, 0, "C H E C K   M A T E");
}

fn render_stalemate(ctx: &mut BTerm) {
    set_active_console_texts(ctx);
    ctx.cls();
    ctx.print(TEXT_LEFT_START, 0, "STALEMATE");
}

pub(super) fn render_promotion_pawn(optional_pawn: Option<Piece>, ctx: &mut BTerm) {
    if let Some(pawn) = optional_pawn {
        set_active_console_board(ctx);
        ctx.set(
            pawn.square.x(),
            pawn.square.y(),
            LIGHT_SALMON,
            to_square_ui_color(pawn.square.color()),
            to_cp437(CHESS_CODE),
        );
    }
}

pub(super) fn render_selected_piece(piece: &Piece, ctx: &mut BTerm) {
    set_active_console_board(ctx);
    ctx.set(
        piece.square.x(),
        piece.square.y(),
        LIGHT_CYAN,
        to_square_ui_color(piece.square.color()),
        to_cp437(CHESS_CODE),
    );
}

fn set_active_console_texts(ctx: &mut BTerm) {
    ctx.set_active_console(2)
}

pub fn render_executed_moves(game: &ChessGame, ctx: &mut BTerm) {
    set_active_console_texts(ctx);
    let mut row = 1;
    game.executed_moves().iter().for_each(|executed_move| {
        let move_number = (row - 1) / 2 + 1;
        let column_offset = 1 - row % 2;
        let string = match column_offset == 0 {
            true => format!("{}. {}", move_number, executed_move.coord_notation()),
            false => executed_move.coord_notation(),
        };
        ctx.print(TEXT_LEFT_START + column_offset * 10, move_number, string);
        row += 1;
    });
}

impl GameState for MainState {
    fn tick(&mut self, ctx: &mut BTerm) {
        render_board(&self.game.board, ctx);
        render_pieces(&self.game.pieces, ctx);
        render_game_end_and_check(&self.game, ctx);
        render_promotion_pawn(self.game.promotion_pawn(), ctx);
        match &self.app_state {
            AppState::AwaitingMoveSelection { user_move } => {
                render_selected_piece(&user_move.piece, ctx)
            }
            AppState::AwaitingPieceSelection => {}
        }
        render_executed_moves(&self.game, ctx);

        set_active_console_pieces(ctx);
        INPUT.lock().for_each_message(|message| {
            let mouse_point = ctx.mouse_point();
            let coord = (mouse_point.x as i8, mouse_point.y as i8);
            match message {
                BEvent::MouseButtonDown { button: 0 } => self.evaluate_mouse_click(coord),
                BEvent::CloseRequested { .. } => ctx.quit(),
                _ => {}
            }
        });

        if let AppState::AwaitingMoveSelection { user_move } = &self.app_state {
            render_possible_moves(user_move.possible_moves.clone(), ctx);
        }
    }
}
