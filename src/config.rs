use eyre::Result;
#[allow(unused_imports)]
use raalog::{debug, error, info, trace, warn};

use clap::Parser;
use std::time::Duration;

// // // // // // // //
pub fn setup() -> Result<AppConfig> {
    let clargs = CliArgs::parse();

    let app_config = AppConfig {
        refresh_delay: Duration::from_millis( 1000_u64 / (clargs.refresh_rate as u64)),
        game_lua_code: try_load_file(&clargs.game, clargs.tab_to_spaces),
        auto_run: clargs.auto_run,
    };
    Ok(app_config)
}

#[derive(Clone)]
pub struct AppConfig {
    pub refresh_delay: Duration,
    pub game_lua_code: String,
    pub auto_run: bool,
}

// // // // // // // //
#[derive(Parser)]
#[command(version, about)]
struct CliArgs {
    #[arg(short, long, default_value = "")]
    game: String,
    #[arg(short, long, default_value_t = false)]
    auto_run: bool,
    #[arg(short, long, default_value_t = 10 )]
    refresh_rate: u8,
    #[arg(long, default_value_t = 4)]
    tab_to_spaces: u8,
}

// // // // // // // //
fn try_load_file(file_name: &str, tab_to_spaces: u8) -> String {
    if file_name.is_empty() {
        warn!("File for the game script is not specified.\nNeeds to create script in-game.");
        return String::new();
    }
    info!("Loading the game script: <{}>..", file_name);

    let game_lua_code = match std::fs::read_to_string(file_name) {
        Ok(content) => {
            let replacer = (0..tab_to_spaces).map(|_|" ").collect::<String>();
            content.replace("\t", &replacer)
        }
        Err(e) => {
            error!("Error during loading the game script from <{}>\n{}", file_name, e.to_string());
            return String::new();
        }
    };

    game_lua_code
}
