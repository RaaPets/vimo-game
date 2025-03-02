use anyhow::Result;

#[allow(unused_imports)]
use raalog::{debug, error, info, trace, warn};

use super::GameCommand;
use super::GameModel;
use super::game_state::GameState;

//  //  //  //  //  //  //  //
impl GameModel {
    #[inline]
    pub(super) fn internal_action(&mut self, act: GameCommand) -> Result<()> {
        match act as i64 {
            index @ 1..=4 => {
                let lua_action: mlua::Function = self.lua.globals().get("action")?;
                match lua_action.call::<_, mlua::Value>(mlua::Value::Integer(index))? {
                    mlua::Value::Nil => return Ok(()),
                    other_type @ _ => {
                        let Ok(s) = other_type.to_string() else {
                            return Err(anyhow::anyhow!("invalid action type result"));
                        };
                        self.game_state = GameState::GameOver(s);
                        return Ok(());
                    }
                }
            }
            wrong_index @ _ => {
                return Err(anyhow::anyhow!("invalid action index <{}>", wrong_index))
            }
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
    fn enum_indexation_2() -> Result<()> {
        let code = r#"
                        function action(act)
                            return act
                        end
                    "#;
        let mut model = GameModel::new(code)?;
        model.internal_action(GameCommand::Up)?;
        assert!(model.game_state == GameState::GameOver("1".to_owned()));

        model.internal_action(GameCommand::Down)?;
        assert!(model.game_state == GameState::GameOver("2".to_owned()));

        model.internal_action(GameCommand::Left)?;
        assert!(model.game_state == GameState::GameOver("3".to_owned()));

        model.internal_action(GameCommand::Right)?;
        assert!(model.game_state == GameState::GameOver("4".to_owned()));

        Ok(())
    }

    #[test]
    fn enum_indexation() -> Result<()> {
        let code = r#"
                        function action(act)
                        end
                    "#;
        let mut model = GameModel::new(code)?;
        model.internal_action(GameCommand::Up)?;
        model.internal_action(GameCommand::Down)?;
        model.internal_action(GameCommand::Left)?;
        model.internal_action(GameCommand::Right)?;
        Ok(())
    }

    #[test]
    fn enum_raw_indexation() -> Result<()> {
        assert!(GameCommand::Up as u8 == 1);
        assert!(GameCommand::Down as u8 == 2);
        assert!(GameCommand::Left as u8 == 3);
        assert!(GameCommand::Right as u8 == 4);
        Ok(())
    }
}
