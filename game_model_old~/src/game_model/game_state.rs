#[derive(PartialEq)]
pub enum GameState {
    Undef,
    Running(GameObjects),
    GameOver(String),
}

#[derive(PartialEq)]
pub struct GameObjects {
    pub(super) player: Option<(u16, u16)>,
    pub(super) target: Option<(u16, u16)>,
    pub(super) obstacles: Vec<(u16, u16)>,
}

#[allow(dead_code)]
#[derive(PartialEq)]
pub enum CellState {
    Empty,
    Player,
    Target,
    Obstacle,
}

impl GameState {
    pub fn cell_state(&self, i: u16, j: u16) -> CellState {
        if let GameState::Running(objs) = &self {
            if objs.player == Some((i, j)) {
                return CellState::Player;
            }
            if objs.target == Some((i, j)) {
                return CellState::Target;
            }
            for obstacle in &objs.obstacles {
                if *obstacle == (i, j) {
                    return CellState::Obstacle;
                }
            }
        }
        CellState::Empty
    }
}

//  //  //  //  //  //  //  //
//        TEST              //
//  //  //  //  //  //  //  //
#[cfg(test)]
mod game_state_tests {
    use super::*;
    use anyhow::Result;

    #[test]
    fn undef_state() -> Result<()> {
        let state = GameState::Undef;
        for i in 0..256 {
            for j in 0..256 {
                let cell_state = state.cell_state(i, j);
                assert!(cell_state == CellState::Empty);
            }
        }
        Ok(())
    }

    #[test]
    fn gameover_state() -> Result<()> {
        let state = GameState::GameOver(String::from("some failure"));
        for i in 0..256 {
            for j in 0..256 {
                let cell_state = state.cell_state(i, j);
                assert!(cell_state == CellState::Empty);
            }
        }
        Ok(())
    }
}
