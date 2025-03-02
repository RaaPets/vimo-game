#[derive(PartialEq)]
pub enum GameState {
    Undef,
    Running(GameObjects),
    GameOver(String, GameObjects),
}

#[derive(Clone, PartialEq)]
pub struct GameObjects {
    pub(super) player: Option<(u16, u16)>,
    pub(super) target: Option<(u16, u16)>,
    pub(super) obstacles: Vec<(u16, u16)>,
}

#[allow(dead_code)]
#[derive(PartialEq)]
pub enum CellState {
    Empty,
    RedEmpty,
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
        if let GameState::GameOver(_, objs) = &self {
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
            return CellState::RedEmpty;
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
    use eyre::Result;

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
        let last_objects = GameObjects {
            player: None,
            target: Some((5,6)),
            obstacles: Vec::new(),
        };
        let state = GameState::GameOver(String::from("some failure"), last_objects);
        for i in 0..256 {
            for j in 0..256 {
                let cell_state = state.cell_state(i, j);
                if i == 5 && j == 6 {
                    assert!(cell_state == CellState::Target);
                } else {
                    assert!(cell_state == CellState::RedEmpty);
                }
            }
        }
        Ok(())
    }
}
