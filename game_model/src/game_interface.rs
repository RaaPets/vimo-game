use eyre::Result;

pub enum GameCommand {
    Up,
    Down,
    Left,
    Right,
}

use crate::prelude::*;
pub trait GameModelInterface {
    fn state(&self) -> &GameState;
    fn update(&mut self, time: i64, cmd: Option<GameCommand>) -> Result<()>;
}
