use eyre::Result;

#[allow(unused_imports)]
use raalog::{debug, error, info, trace, warn};

use super::game_state::GameObjects;
use super::game_state::GameState;
use super::GameModel;

//  //  //  //  //  //  //  //
impl GameModel {
    #[inline]
    pub(super) fn invoke_lua_update(
        &self,
        time: i64,
        player: Option<(u16, u16)>,
    ) -> Result<GameState> {
        let lua_update: mlua::Function = self.lua.globals().get("update")?;
        let update_result = self.get_update_result(lua_update, time, player)?;

        if let Ok(s) = update_result.get::<&str, String>("GameOver") {
            if let GameState::Running(obj) = &self.game_state {
                return Ok(GameState::GameOver(s.clone(), obj.clone()));
            } else {
                return Ok(GameState::GameOver(s.clone(), GameObjects {
                    player: None, target: None, obstacles: Vec::new(),
                }));
            }
        }

        let objects = GameObjects {
            player: player,
            target: extract_xy_byname(&update_result, "target"),
            obstacles: extract_list(&update_result, "obstacles"),
        };
        return Ok(GameState::Running(objects));
    }

    fn get_update_result<'a>(
        &'a self,
        lua_update: mlua::Function<'a>,
        time: i64,
        player: Option<(u16, u16)>,
    ) -> Result<mlua::Table<'a>> {
        match player {
            None => Ok(lua_update.call::<_, mlua::Table>(mlua::Value::Integer(time))?),
            Some(pl) => {
                let pl_tbl = self.lua.create_table()?;
                pl_tbl.set(1, pl.0)?;
                pl_tbl.set(2, pl.1)?;
                Ok(lua_update.call::<_, mlua::Table>((
                    mlua::Value::Integer(time),
                    mlua::Value::Table(pl_tbl),
                ))?)
            }
        }
    }
}

//  //  //  //  //  //  //  //
fn extract_xy_byname(tbl: &mlua::Table, name: &str) -> Option<(u16, u16)> {
    let Ok(pos) = tbl.get::<&str, mlua::Table>(name) else {
        return None;
    };
    let Ok(x) = pos.get::<u16, u16>(1) else {
        return None;
    };
    let Ok(y) = pos.get::<u16, u16>(2) else {
        return None;
    };
    Some((x, y))
}

#[inline]
fn extract_xy_byindex(tbl: &mlua::Table, index: i64) -> Option<(u16, u16)> {
    let Ok(pos) = tbl.get::<i64, mlua::Table>(index) else {
        return None;
    };
    let Ok(x) = pos.get::<u16, u16>(1) else {
        return None;
    };
    let Ok(y) = pos.get::<u16, u16>(2) else {
        return None;
    };
    Some((x, y))
}

fn extract_list(update_result: &mlua::Table, name: &str) -> Vec<(u16, u16)> {
    let mut list = Vec::new();
    let Ok(items) = update_result.get::<&str, mlua::Table>(name) else {
        return list;
    };
    let Ok(len) = items.len() else {
        return list;
    };

    for i in 1_i64..=len {
        let Some(xy) = extract_xy_byindex(&items, i) else {
            continue;
        };
        list.push(xy);
    }

    list
}

//  //  //  //  //  //  //  //
//        TESTS             //
//  //  //  //  //  //  //  //
#[cfg(test)]
mod game_model_tests {
    use super::*;

    #[test]
    fn game_over() -> Result<()> {
        let code = r#"
                        function update(time)
                        return {
                            GameOver = "Some reason",
                        }
                        end
                    "#;
        let model = GameModel::new(code)?;
        let new_state = model.invoke_lua_update(-1, None)?;
        match new_state {
            GameState::Undef => Err(eyre::eyre!("can't be GameState::Undef")),
            GameState::GameOver(_, _) => Ok(()),
            GameState::Running(_) => Err(eyre::eyre!("can't be GameState::Running()")),
        }
    }

    #[test]
    fn all_in() -> Result<()> {
        let code = r#"
                        function update(time, player)
                            if player == nil then
                                error("<player> is NIL")
                            end
                            if player[1] ~= 11 then
                                error("<player> with wrong [1]")
                            end
                            if player[2] ~= 7 then
                                error("<player> with wrong [2]")
                            end
                        return {
                            obstacles = {
                                {3,14},{4,15},
                            },
                            target = {2,6},
                        }
                        end
                    "#;
        let model = GameModel::new(code)?;
        let new_state = model.invoke_lua_update(-1, Some((11, 7)))?;
        match new_state {
            GameState::Undef => Err(eyre::eyre!("can't be GameState::Undef")),
            GameState::GameOver(_, _) => Err(eyre::eyre!("can't be GameState::GameOver()")),
            GameState::Running(objs) => {
                assert!(objs.player == Some((11, 7)));
                assert!(objs.target == Some((2, 6)));
                let len = objs.obstacles.len();
                assert!(len == 2, "invalid len() - {}", len);
                assert!(objs.obstacles[0] == (3, 14));
                assert!(objs.obstacles[1] == (4, 15));
                Ok(())
            }
        }
    }

    #[test]
    fn only_obstacles() -> Result<()> {
        let code = r#"
                        function update(time)
                        return {
                            obstacles = {
                                {13,4},{14,4},{14,5},
                            },
                        }
                        end
                    "#;
        let model = GameModel::new(code)?;
        let new_state = model.invoke_lua_update(-1, None)?;
        match new_state {
            GameState::Undef => Err(eyre::eyre!("can't be GameState::Undef")),
            GameState::GameOver(_, _) => Err(eyre::eyre!("can't be GameState::GameOver()")),
            GameState::Running(objs) => {
                assert!(objs.target.is_none());
                assert!(objs.player.is_none());
                let len = objs.obstacles.len();
                assert!(len == 3, "invalid len() - {}", len);
                assert!(objs.obstacles[0] == (13, 4));
                assert!(objs.obstacles[1] == (14, 4));
                assert!(objs.obstacles[2] == (14, 5));
                Ok(())
            }
        }
    }

    #[test]
    fn only_obstacles_single() -> Result<()> {
        let code = r#"
                        function update(time)
                        return {
                            obstacles = {
                                {5,6},
                            },
                        }
                        end
                    "#;
        let model = GameModel::new(code)?;
        let new_state = model.invoke_lua_update(-1, None)?;
        match new_state {
            GameState::Undef => Err(eyre::eyre!("can't be GameState::Undef")),
            GameState::GameOver(_, _) => Err(eyre::eyre!("can't be GameState::GameOver()")),
            GameState::Running(objs) => {
                assert!(objs.target.is_none());
                assert!(objs.player.is_none());
                let len = objs.obstacles.len();
                assert!(len == 1, "invalid len() - {}", len);
                assert!(objs.obstacles[0] == (5, 6));
                Ok(())
            }
        }
    }

    #[test]
    fn only_empty_obstacles_count() -> Result<()> {
        let code = r#"
                        function update(time)
                        return {
                            obstacles = {
                            },
                        }
                        end
                    "#;
        let model = GameModel::new(code)?;
        let new_state = model.invoke_lua_update(-1, None)?;
        match new_state {
            GameState::Undef => Err(eyre::eyre!("can't be GameState::Undef")),
            GameState::GameOver(_, _) => Err(eyre::eyre!("can't be GameState::GameOver()")),
            GameState::Running(objs) => {
                assert!(objs.target.is_none());
                assert!(objs.player.is_none());
                let len = objs.obstacles.len();
                assert!(len == 0, "invalid len() - {}", len);
                Ok(())
            }
        }
    }

    #[test]
    fn only_target_position() -> Result<()> {
        let code = r#"
                        function update(time)
                        return {
                            target = {13,14},
                        }
                        end
                    "#;
        let model = GameModel::new(code)?;
        let new_state = model.invoke_lua_update(-1, None)?;
        match new_state {
            GameState::Undef => Err(eyre::eyre!("can't be GameState::Undef")),
            GameState::GameOver(_, _) => Err(eyre::eyre!("can't be GameState::GameOver()")),
            GameState::Running(objs) => {
                assert!(objs.target == Some((13, 14)));
                assert!(objs.player.is_none());
                assert!(objs.obstacles.is_empty());
                Ok(())
            }
        }
    }

    #[test]
    fn new_empty_objects() -> Result<()> {
        let code = r#"
                        function update(time)
                        return {}
                        end
                    "#;
        let model = GameModel::new(code)?;
        let new_state = model.invoke_lua_update(-1, None)?;
        match new_state {
            GameState::Undef => Err(eyre::eyre!("can't be GameState::Undef")),
            GameState::GameOver(_, _) => Err(eyre::eyre!("can't be GameState::GameOver()")),
            GameState::Running(objs) => {
                assert!(objs.player.is_none());
                assert!(objs.target.is_none());
                assert!(objs.obstacles.is_empty());
                Ok(())
            }
        }
    }
}
