use anyhow::Result;

#[allow(unused_imports)]
use raalog::{debug, error, info, trace, warn};

use crate::lua_connector;
use crate::prelude::*;

//  //  //  //  //  //  //  //
mod game_state;
mod impl_action;
mod impl_update;

pub use game_state::*;

pub struct GameModel {
    lua: mlua::Lua,
    pub(crate) game_state: GameState,
}

impl GameModel {
    pub fn new(code: &str) -> Result<Self> {
        let new_one = Self {
            lua: lua_connector::init(code)?,
            game_state: GameState::Undef,
        };

        trace!(" + GameModel::new()");
        Ok(new_one)
    }
}
impl Drop for GameModel {
    fn drop(&mut self) {
        self.game_state = GameState::Undef;
        trace!(" - GameModel::drop()");
    }
}

impl GameModelInterface for GameModel {
    fn state(&self) -> &GameState {
        &self.game_state
    }
    fn update(&mut self, time: i64) -> Result<()> {
        if let GameState::GameOver(_) = self.game_state {
            Ok(())
        } else {
            self.internal_update(time)
        }
    }

    fn action(&mut self, act: GameCommand) -> Result<()> {
        if let GameState::GameOver(_) = self.game_state {
            Ok(())
        } else {
            self.internal_action(act)
        }
    }
}

//  //  //  //  //  //  //  //
//        TEST              //
//  //  //  //  //  //  //  //
#[cfg(test)]
mod game_model_tests {
    use super::*;

    #[test]
    fn new_undef_state() -> Result<()> {
        let code = "";
        let model = GameModel::new(code)?;
        assert!(model.game_state == GameState::Undef);
        Ok(())
    }

    #[test]
    fn new_update_error() -> Result<()> {
        let code = "";
        let mut model = GameModel::new(code)?;
        let r = model.update(-1);
        assert!(r.is_err());
        Ok(())
    }
}
