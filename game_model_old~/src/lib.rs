mod game_interface;

mod game_model;
mod lua_connector;


pub mod prelude {
    pub use crate::game_interface::{GameModelInterface, GameCommand};
    pub use crate::game_model::{GameModel, GameState, CellState};
}
