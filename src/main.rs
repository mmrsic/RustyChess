use crate::domain::game::ChessGame;
use crate::domain::pieces::{PieceColor, PieceType};
use crate::ui::user_move::UserMove;

mod domain;
mod ui;

/** Main entry point of the application. */
fn main() {
    ui::main(MainState::new()).unwrap()
}

/** All possible states of the application. */
#[derive(Debug)]
enum AppState {
    AwaitingPieceSelection,
    AwaitingMoveSelection { user_move: UserMove },
}

/** Main state consisting of the chess game and the state of the application. */
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

    fn evaluate_mouse_click(&mut self, coord: (i8, i8)) {
        match &self.app_state {
            AppState::AwaitingPieceSelection => {
                if let Some(selected_piece) = self.game.piece_at(coord) {
                    let promo_pawn = self.game.promotion_pawn();
                    if promo_pawn.is_some() {
                        if selected_piece.square == promo_pawn.unwrap().square {
                            self.game.exchange_promotion_pawn(PieceType::Queen);
                            return;
                        }
                    }
                    let possible_moves = self.game.possible_moves(selected_piece);
                    self.app_state = AppState::AwaitingMoveSelection {
                        user_move: UserMove::new(selected_piece.clone(), possible_moves),
                    };
                } else if let Some(square) = self.game.board.square_at(coord) {
                    let challengers = self.game.square_challengers(square);
                    println!("Challengers for {}: {:?}", square, challengers);
                    println!("Can move = {:?}", self.game.can_move(&PieceColor::White));
                }
            }
            AppState::AwaitingMoveSelection { user_move } => {
                if let Some(selected_target) = self.game.board.square_at(coord) {
                    if let Some(chosen_move) = user_move.possible_move_to_target(*selected_target) {
                        self.game.execute_move(chosen_move);
                    }
                }
                self.app_state = AppState::AwaitingPieceSelection;
            }
        }
    }
}
