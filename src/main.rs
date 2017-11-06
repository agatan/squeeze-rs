extern crate termfest;

use std::thread;
use std::sync::{Arc, Mutex};
use std::sync::mpsc;

use termfest::Event;

mod sq;
mod ui;

fn main() {
    let (term, events) = termfest::Termfest::hold().expect("failed to initialize terminal");
    let (render_tx, render_rx) = mpsc::channel();
    let sq = Arc::new(Mutex::new(sq::Sq::new(sq::mode::Normal, render_tx)));
    let ui = Arc::new(ui::Ui::new(sq.clone(), term));
    get_async_candidates(sq.clone());
    let tmp = ui.clone();
    thread::spawn(move || rendering(tmp, render_rx));
    if let Some(r) = event_poller(ui, events) {
        println!("{}", r);
    }
}

fn get_async_candidates(sq: Arc<Mutex<sq::Sq>>) {
    let _ = thread::spawn(move || {
        let stdin = std::io::stdin();
        loop {
            let mut buf = String::new();
            match stdin.read_line(&mut buf) {
                Ok(0) => return,
                Ok(_) => {}
                Err(e) => panic!("{}", e),
            }
            let mut sq = sq.lock().unwrap();
            sq.push_candidate(buf);
        }
    });
}

fn event_poller(ui: Arc<ui::Ui>, events: mpsc::Receiver<Event>) -> Option<String> {
    for ev in events {
        if let Some(r) = ui.event_handler(ev) {
            return Some(r);
        }
    }
    return None;
}

fn rendering(ui: Arc<ui::Ui>, trigger: mpsc::Receiver<()>) {
    for () in trigger {
        ui.render();
    }
}
