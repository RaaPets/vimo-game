use anyhow::Result;

#[allow(unused_imports)]
use raalog::{debug, error, info, trace, warn};

use game_model::prelude::*;

static START_COMMANDS: &str = "10j3k2l\n3h\n6j7l3k";

//  //  //  //  //  //  //  //
pub struct AppModel {
    pub(crate) tick_counter: u16,
    pub(crate) game_time: i64,
    pub(crate) game_actions: Vec<char>,
    pub(super) game: Option<GameModel>,
    pub(super) game_editor_state: edtui::EditorState,
    pub(super) game_editor_handler: edtui::EditorEventHandler,
    pub(super) command_editor_state: edtui::EditorState,
    pub(super) ed_handler: edtui::EditorEventHandler,
    pub(super) state: AppModelState,
    pub(crate) is_popup: bool,
    pub(crate) config: crate::config::AppConfig,
}

struct EdTuiHolder {
    state: edtui::EditorState,
    handler: edtui::EditorEventHandler,
}

#[derive(PartialEq)]
pub enum AppModelState {
    //EditorFocused,
    OffFocused,
    Exiting,
}

impl AppModel {
    pub fn new(config: &crate::config::AppConfig) -> Result<Self> {
        let app = Self {
            tick_counter: 0,
            game_time: 0,
            game_actions: Vec::new(),
            game: None,
            game_editor_state: edtui::EditorState::new(edtui::Lines::from(config.game_lua_code.as_str())),
            game_editor_handler: edtui::EditorEventHandler::default(),
            command_editor_state: edtui::EditorState::new(edtui::Lines::from(START_COMMANDS)),
            ed_handler: edtui::EditorEventHandler::default(),
            state: AppModelState::OffFocused,
            is_popup: false,
            config: config.clone(),
        };

        trace!(" + AppModel::new()");
        Ok(app)
    }

    pub fn is_exiting(&self) -> bool {
        self.state == AppModelState::Exiting
    }
}
