use bracket_lib::prelude::{main_loop, BError};

use crate::ui::rendering::create_gui;
use crate::MainState;

pub mod rendering;
pub mod user_move;

pub(super) fn main(main_state: MainState) -> BError {
    main_loop(create_gui(), main_state)
}
