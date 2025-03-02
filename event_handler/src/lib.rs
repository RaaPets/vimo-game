use eyre::Result;
#[allow(unused_imports)]
use raalog::{debug, error, info, trace, warn};

use crossterm::event as xEvent;

//  //  //  //  //  //  //  //
pub enum Events {
    Tick,
    Exit,
    Input(xEvent::Event),
}

pub struct EventHandler {
    rx: std::sync::mpsc::Receiver<Result<Events>>,
}

impl EventHandler {
    pub fn new(tick_duration: std::time::Duration) -> Self {
        let (tx, rx) = std::sync::mpsc::channel();
        let tx_clone = tx.clone();
        std::thread::spawn(move || loop {
            let raw_event = block_til_event();
            let result_event = preprocess_event(raw_event);
            let Ok(()) = tx.send(result_event) else {
                return;
            };
        });
        std::thread::spawn(move || loop {
            std::thread::sleep(tick_duration);
            let Ok(()) = tx_clone.send(Ok(Events::Tick)) else {
                return;
            };
        });

        Self { rx }
    }

    pub fn wait_next(&self) -> Result<Events> {
        self.rx.recv()?
    }
}

//  //  //  //  //  //  //  //
#[inline(always)]
fn block_til_event() -> Result<xEvent::Event> {
    Ok(xEvent::read()?)
}

#[inline(always)]
fn preprocess_event(raw_event: Result<xEvent::Event>) -> Result<Events> {
    match raw_event {
        Ok(ev) => {
            if let Some(term_ev) = check_terminate_sequence(&ev) {
                return Ok(term_ev);
            } else {
                return Ok(Events::Input(ev));
            }
        }
        Err(e) => Err(e),
    }
}

//  //  //  //  //  //  //  //
fn check_terminate_sequence(event: &xEvent::Event) -> Option<Events> {
    match event {
        xEvent::Event::Key(key) => {
            if key.modifiers.contains(xEvent::KeyModifiers::CONTROL) {
                // <C-c>
                if key.code == xEvent::KeyCode::Char('c') {
                    let msg = "exiting by <C-c>";
                    warn!("{}", msg);
                    return Some(Events::Exit);
                }
                // TODO: doesn't work <C-/>
                if key.code == xEvent::KeyCode::Char('/') {
                    let msg = "aborted by <C-/>";
                    error!("{}", msg);
                    panic!("{}", msg);
                }
            }
        }
        _ => {}
    }
    None
}
