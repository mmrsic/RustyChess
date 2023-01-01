use bracket_lib::prelude::*;

use crate::chess_game::*;
use crate::move_rules::*;
use crate::rendering::*;
use crate::user_move::*;

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
                    self.app_state = AppState::AwaitingMoveSelection {
                        user_move: UserMove::new(
                            selected_piece.clone(),
                            create_basic_possible_moves(selected_piece, &self.game.board),
                        ),
                    };
                }
            }
            AppState::AwaitingMoveSelection { user_move } => {
                if let Some(selected_target) = self.game.board.square_at(coord) {
                    if let Some(chosen_move) = user_move
                        .possible_moves
                        .iter()
                        .find(|possible_move| possible_move.target == *selected_target)
                    {
                        if let Some(target_piece) =
                            self.game.piece_at(chosen_move.target.position())
                        {
                            CapturingMove::new(chosen_move.piece.clone(), target_piece.clone())
                                .execute(&mut self.game);
                        } else {
                            Move::new(chosen_move.piece.clone(), chosen_move.target)
                                .execute(&mut self.game);
                        }
                    }
                }
                self.app_state = AppState::AwaitingPieceSelection;
            }
            AppState::NoActionRequired => {}
        }
    }
}

#[derive(Debug)]
enum AppState {
    NoActionRequired,
    AwaitingPieceSelection,
    AwaitingMoveSelection { user_move: UserMove },
}

impl GameState for MainState {
    fn tick(&mut self, ctx: &mut BTerm) {
        render_board(&self.game.board, ctx);
        render_pieces(&self.game.pieces, ctx);

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
