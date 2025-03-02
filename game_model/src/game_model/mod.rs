use eyre::Result;

#[allow(unused_imports)]
use raalog::{debug, error, info, trace, warn};

use crate::lua_connector;
use crate::prelude::*;

//  //  //  //  //  //  //  //
mod game_state;
mod impl_invoke_lua_update;
mod move_player;

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

    fn update(&mut self, time: i64, opt_cmd: Option<GameCommand>) -> Result<()> {
        match (&self.game_state, time) {
            (_, -1) => {
                let new_player = self.lua_read_player()?;
                self.game_state = self.invoke_lua_update(time, new_player)?;
                Ok(())
            }
            (GameState::Undef, _) => Err(eyre::eyre!("The first time stamp in Undef state must be <-1>")),
            (GameState::GameOver(_, _), _) => Ok(()),
            (GameState::Running(objs), _) => {
                let new_player = move_player::move_player(objs.player, opt_cmd);
                self.game_state = self.invoke_lua_update(time, new_player)?;
                Ok(())
            }
        }
    }
}

impl GameModel {
    fn lua_read_player(&self) -> Result<Option<(u16,u16)>> {
        let lua_new_player: mlua::Table= self.lua.globals().get("new_player")?;
        let Ok(x) = lua_new_player.get::<u16, u16>(1) else {
            return Ok(None);
        };
        let Ok(y) = lua_new_player.get::<u16, u16>(2) else {
            return Ok(None);
        };
        Ok(Some((x, y)))
    }
}

//  //  //  //  //  //  //  //
//        TEST              //
//  //  //  //  //  //  //  //
#[cfg(test)]
mod game_model_tests {
    use super::*;

    #[test]
    fn new_player_position() -> Result<()> {
        let code = r#"
                        new_player = {7,4}
                        function update(time)
                            return {}
                        end
                    "#;
        let mut model = GameModel::new(code)?;
        assert!(model.game_state == GameState::Undef);
        model.update(-1, None)?;
        let GameState::Running(objs) = &model.game_state else {
            return Err(eyre::eyre!("game_state is not Running(_)"));
        };
        let Some(player) = objs.player else {
            return Err(eyre::eyre!("game_state.player is None"));
        };
        assert!(player == (7,4));
        Ok(())
    }

    #[test]
    fn there_is_no_new_player_error() -> Result<()> {
        let code = r#"
                        new_player = nil
                    "#;
        let mut model = GameModel::new(code)?;
        assert!(model.game_state == GameState::Undef);
        let r = model.update(-1, None);
        assert!(r.is_err());
        Ok(())
    }

    #[test]
    fn new_state_is_undef() -> Result<()> {
        let code = "";
        let model = GameModel::new(code)?;
        assert!(model.game_state == GameState::Undef);
        Ok(())
    }

    #[test]
    fn undef_with_incorrect_time_error() -> Result<()> {
        let code = "";
        let mut model = GameModel::new(code)?;
        let r = model.update(0, None);
        assert!(r.is_err());
        Ok(())
    }

    #[test]
    fn there_is_no_update_error() -> Result<()> {
        let code = "";
        let mut model = GameModel::new(code)?;
        let r = model.update(-1, None);
        assert!(r.is_err());
        Ok(())
    }
}
