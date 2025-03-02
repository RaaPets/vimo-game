use anyhow::Result;

#[allow(unused_imports)]
use raalog::{debug, error, info, trace, warn};

use super::GameModel;
use super::game_state::GameObjects;
use super::game_state::GameState;

//  //  //  //  //  //  //  //
impl GameModel {
    #[inline]
    pub(super) fn internal_update(&mut self, time: i64) -> Result<()> {
        let update: mlua::Function = self.lua.globals().get("update")?;
        let update_result: mlua::Table =
            update.call::<_, mlua::Table>(mlua::Value::Integer(time))?;

        if let Ok(s) = update_result.get::<&str, String>("GameOver") {
            self.game_state = GameState::GameOver(s.clone());
            return Ok(());
        }
        {
            let objects = GameObjects {
                player: extract_xy_byname(&update_result, "player"),
                target: extract_xy_byname(&update_result, "target"),
                obstacles: extract_list(&update_result, "obstacles"),
            };

            self.game_state = GameState::Running(objects);
            Ok(())
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
    fn check_game_over() -> Result<()> {
        let code = r#"
                        function update(time)
                            return {
                                GameOver,
                            }
                        end
                    "#;
        let mut model = GameModel::new(code)?;
        model.internal_update(-1)?;
        match &model.game_state {
            GameState::Undef => Err(anyhow::anyhow!("can't be GameState::Undef")),
            GameState::GameOver(msg) => {
                assert!(msg == "FIN");
                Ok(())
            }
            GameState::Running(_) => Err(anyhow::anyhow!("can't be GameState::Runnint(_)"))
        }
    }

    #[test]
    fn all_in() -> Result<()> {
        let code = r#"
                        function update(time)
                        return {
                            player = {11,7},
                            obstacles = {
                                {3,14},{4,15},
                            },
                            target = {2,6},
                        };
                        end
                    "#;
        let mut model = GameModel::new(code)?;
        model.internal_update(-1)?;
        match &model.game_state {
            GameState::Undef => Err(anyhow::anyhow!("can't be GameState::Undef")),
            GameState::GameOver(_) => Err(anyhow::anyhow!("can't be GameState::GameOver()")),
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
                        };
                        end
                    "#;
        let mut model = GameModel::new(code)?;
        model.internal_update(-1)?;
        match &model.game_state {
            GameState::Undef => Err(anyhow::anyhow!("can't be GameState::Undef")),
            GameState::GameOver(_) => Err(anyhow::anyhow!("can't be GameState::GameOver()")),
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
                        };
                        end
                    "#;
        let mut model = GameModel::new(code)?;
        model.internal_update(-1)?;
        match &model.game_state {
            GameState::Undef => Err(anyhow::anyhow!("can't be GameState::Undef")),
            GameState::GameOver(_) => Err(anyhow::anyhow!("can't be GameState::GameOver()")),
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
                        };
                        end
                    "#;
        let mut model = GameModel::new(code)?;
        model.internal_update(-1)?;
        match &model.game_state {
            GameState::Undef => Err(anyhow::anyhow!("can't be GameState::Undef")),
            GameState::GameOver(_) => Err(anyhow::anyhow!("can't be GameState::GameOver()")),
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
                        };
                        end
                    "#;
        let mut model = GameModel::new(code)?;
        model.internal_update(-1)?;
        match &model.game_state {
            GameState::Undef => Err(anyhow::anyhow!("can't be GameState::Undef")),
            GameState::GameOver(_) => Err(anyhow::anyhow!("can't be GameState::GameOver()")),
            GameState::Running(objs) => {
                assert!(objs.target == Some((13, 14)));
                assert!(objs.player.is_none());
                assert!(objs.obstacles.is_empty());
                Ok(())
            }
        }
    }

    #[test]
    fn only_player_position() -> Result<()> {
        let code = r#"
                        function update(time)
                        return {
                            player = {3,4},
                        };
                        end
                    "#;
        let mut model = GameModel::new(code)?;
        model.internal_update(-1)?;
        match &model.game_state {
            GameState::Undef => Err(anyhow::anyhow!("can't be GameState::Undef")),
            GameState::GameOver(_) => Err(anyhow::anyhow!("can't be GameState::GameOver()")),
            GameState::Running(objs) => {
                assert!(objs.player == Some((3, 4)));
                assert!(objs.target.is_none());
                assert!(objs.obstacles.is_empty());
                Ok(())
            }
        }
    }

    #[test]
    fn new_empty_objects() -> Result<()> {
        let code = r#"
                        function update(time)
                        return {};
                        end
                    "#;
        let mut model = GameModel::new(code)?;
        model.internal_update(-1)?;
        match &model.game_state {
            GameState::Undef => Err(anyhow::anyhow!("can't be GameState::Undef")),
            GameState::GameOver(_) => Err(anyhow::anyhow!("can't be GameState::GameOver()")),
            GameState::Running(objs) => {
                assert!(objs.player.is_none());
                assert!(objs.target.is_none());
                assert!(objs.obstacles.is_empty());
                Ok(())
            }
        }
    }
}
