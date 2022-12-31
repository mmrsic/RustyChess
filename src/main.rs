use bracket_lib::prelude::*;

use crate::chess_game::ChessGame;
use crate::pieces::Piece;
use crate::rendering::*;
use crate::UserInputState::{AwaitingMoveSelection, AwaitingPieceSelection};

mod chess_game;
mod chessboard;
mod move_rules;
mod pieces;
mod rendering;

fn main() -> BError {
    main_loop(create_gui(), MainState::new())
}

struct MainState {
    game: ChessGame,
    user_input_state: Option<UserInputState>,
    selected_piece: Option<Piece>,
}

impl MainState {
    fn new() -> Self {
        Self {
            game: ChessGame::new(),
            user_input_state: Option::from(AwaitingPieceSelection),
            selected_piece: None,
        }
    }
}

enum UserInputState {
    AwaitingPieceSelection,
    AwaitingMoveSelection,
}

impl GameState for MainState {
    fn tick(&mut self, ctx: &mut BTerm) {
        render_board(&self.game.board, ctx);
        render_pieces(&self.game.pieces, ctx);
        match &self.user_input_state {
            Some(AwaitingPieceSelection) => {
                if ctx.left_click && ctx.mouse_visible {
                    if let Some(selected_piece) = self.game.piece_at(ctx.mouse_point()) {
                        self.user_input_state = Option::from(AwaitingMoveSelection);
                        self.selected_piece = Some(selected_piece.clone());
                    }
                }
            }
            Some(AwaitingMoveSelection) => {
                if ctx.left_click && ctx.mouse_visible {
                    if let None = self.game.piece_at(ctx.mouse_point()) {
                        self.user_input_state = Option::from(AwaitingPieceSelection);
                        self.selected_piece = None;
                    }
                } else {
                    render_possible_moves(
                        self.selected_piece.as_ref().unwrap(),
                        &self.game.board,
                        ctx,
                    );
                }
            }
            None => {}
        }
    }
}
