use eyre::Result;

#[allow(unused_imports)]
use raalog::{debug, error, info, trace, warn};

//  //  //  //  //  //  //  //
pub struct App {
    exiting: bool,
}

impl App {
    pub fn new() -> Result<Self> {
        let app = App { exiting: false };
        trace!(" + App::new()");
        Ok(app)
    }

    pub fn run(&mut self) -> Result<()> {
        let mut terminal = ratatui::init();
        let result = self.internal_run_loop(&mut terminal);
        ratatui::restore();
        if let Err(ref e) = result {
            error!("{}", e);
        }
        result
    }

    fn internal_run_loop(
        &mut self,
        _terminal: &mut ratatui::Terminal<impl ratatui::prelude::Backend>,
    ) -> Result<()> {
        return Err(eyre::eyre!("test Error"));
    }
}

impl Drop for App {
    fn drop(&mut self) {
        if self.exiting {
            trace!(" - App::drop() with True Exiting");
        } else {
            trace!(" - App::drop() with False Exiting");
        }
    }
}

//  //  //  //  //  //  //  //
//        TEST              //
//  //  //  //  //  //  //  //
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run() -> Result<()> {
        let mut app = App::new()?;
        app.run();
        Ok(())
    }
}
