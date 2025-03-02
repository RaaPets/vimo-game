use eyre::Result;
#[allow(unused_imports)]
use raalog::{debug, error, info, trace, warn};

//mod app;
//use app as old_app;
mod config;

use app_raa_tui::App as App;

//  //  //  //  //  //  //  //
fn main() -> Result<()> {
    log_init();

    //let app_config = config::setup()?;

    let mut app = App::new()?;
    let result = app
                    .run();
    //let mut terminal = ratatui::init();
    //let result = old_app::run(&mut terminal, &app_config);
    //ratatui::restore();

    //if let Err(ref e) = result {
    //    error!("{}", e);
    //}

    trace!("############\n<-----\n.\n ");
    result
}

//  //  //  //  //  //  //  //
fn log_init() {
    raalog::init()
        .expect("unable init log system")
        .set_file_mode(&"/tmp/rust_debug.log")
        .expect("unable to set file mode of logger")
        .set_level(raalog::LevelFilter::Trace);

    trace!("\n.\n----->\n############");
    set_panic_hook();
}

//  //  //  //  //  //  //  //
fn set_panic_hook() {
    let hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        error!("############\nFATAL!\n{}\n<-----\n.\n ", info);
        hook(info);
    }));
}
