
use crate::prelude::*;

//  //  //  //  //  //  //  //
pub(crate) fn move_player(opt_player: Option<(u16, u16)>, opt_cmd: Option<GameCommand>) -> Option<(u16, u16)> {
    match (opt_player, opt_cmd) {
        (Some(player), Some(command)) => match command {
            GameCommand::Up => Some((player.0, 0b1111 & player.1.overflowing_sub(1).0)),
            GameCommand::Down => Some((player.0, 0b1111 & (player.1 + 1))),
            GameCommand::Left => Some((0b1111 & player.0.overflowing_sub(1).0, player.1)),
            GameCommand::Right => Some((0b1111 & (player.0 + 1), player.1)),
        },
        (_, None) | (None, _) => opt_player,
    }
}

//  //  //  //  //  //  //  //
//        TEST              //
//  //  //  //  //  //  //  //
#[cfg(test)]
mod move_player_tests {
    use super::*;
    use eyre::Result;

    #[test]
    fn no_moves() -> Result<()> {
        assert!(None == move_player(None, None));
        assert!(Some((2, 2)) == move_player(Some((2, 2)), None));
        Ok(())
    }

    #[test]
    fn basic_moves() -> Result<()> {
        assert!(Some((2, 1)) == move_player(Some((2, 2)), Some(GameCommand::Up)));
        assert!(Some((2, 3)) == move_player(Some((2, 2)), Some(GameCommand::Down)));
        assert!(Some((1, 2)) == move_player(Some((2, 2)), Some(GameCommand::Left)));
        assert!(Some((3, 2)) == move_player(Some((2, 2)), Some(GameCommand::Right)));
        Ok(())
    }

    #[test]
    fn edge_moves() -> Result<()> {
        assert!(Some((0, 15)) == move_player(Some((0, 0)), Some(GameCommand::Up)));
        assert!(Some((0, 0)) == move_player(Some((0, 15)), Some(GameCommand::Down)));
        assert!(Some((15, 0)) == move_player(Some((0, 0)), Some(GameCommand::Left)));
        assert!(Some((0, 0)) == move_player(Some((15, 0)), Some(GameCommand::Right)));
        Ok(())
    }
}
