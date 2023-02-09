use bracket_lib::prelude::*;

use crate::chess_game::*;
use crate::rendering::*;
use crate::user_move::*;

mod analysis;
mod chess_game;
mod chessboard;
mod move_rules;
mod pieces;
mod rendering;
mod user_move;

fn main() -> BError {
    main_loop(create_gui(), MainState::new())
}

struct MainState {
    game: ChessGame,
    app_state: AppState,
}

impl MainState {
    fn new() -> Self {
        Self {
            game: ChessGame::new(),
            app_state: AppState::AwaitingPieceSelection,
        }
    }

    fn evaluate_mouse_click(&mut self, coord: Point) {
        match &self.app_state {
            AppState::AwaitingPieceSelection => {
                if let Some(selected_piece) = self.game.piece_at(coord) {
                    let possible_moves = self.game.possible_moves(selected_piece);
                    self.app_state = AppState::AwaitingMoveSelection {
                        user_move: UserMove::new(selected_piece.clone(), possible_moves),
                    };
                } else if let Some(square) = self.game.board.square_at(coord) {
                    let contesters = self.game.square_contesters(square);
                    println!("Contesters for {}: {:?}", square, contesters);
                }
            }
            AppState::AwaitingMoveSelection { user_move } => {
                if let Some(selected_target) = self.game.board.square_at(coord) {
                    if let Some(chosen_move) = user_move.move_to_target(*selected_target) {
                        self.game.execute_move(chosen_move);
                        println!("{:?}", self.game.executed_moves().last().unwrap());
                    }
                }
                self.app_state = AppState::AwaitingPieceSelection;
            }
        }
    }
}

#[derive(Debug)]
enum AppState {
    AwaitingPieceSelection,
    AwaitingMoveSelection { user_move: UserMove },
}

impl GameState for MainState {
    fn tick(&mut self, ctx: &mut BTerm) {
        render_board(&self.game.board, ctx);
        render_pieces(&self.game.pieces, ctx);
        render_chess(&self.game, ctx);
        render_executed_moves(&self.game, ctx);

        set_active_console_pieces(ctx);
        INPUT.lock().for_each_message(|m| match m {
            BEvent::MouseButtonDown { button: 0 } => self.evaluate_mouse_click(ctx.mouse_point()),
            BEvent::CloseRequested { .. } => ctx.quit(),
            _ => {}
        });

        if let AppState::AwaitingMoveSelection { user_move } = &self.app_state {
            render_possible_moves(user_move.possible_moves.clone(), ctx);
        }
    }
}
