use anyhow::Result;

pub enum GameCommand {
    Up = 1,
    Down = 2,
    Left = 3,
    Right = 4,
}

use crate::prelude::*;
pub trait GameModelInterface {
    fn state(&self) -> &GameState;
    fn update(&mut self, time: i64) -> Result<()>;
    fn action(&mut self, act: GameCommand) -> Result<()>;
}
