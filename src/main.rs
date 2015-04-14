#![feature(core)]
#![feature(collections)]

extern crate rustbox;

use std::thread;
use std::sync::{Arc, Mutex};
use std::sync::mpsc;

use rustbox::{RustBox};

mod sq;
mod ui;

fn main() {
    let ui = ui::Ui::new();
    let input = Arc::new(Mutex::new(String::new()));
    let sq = Arc::new(Mutex::new(sq::Sq::new(sq::mode::Normal)));
    let (intr_tx, intr_rx) = mpsc::channel();
    get_async_candidates(sq.clone(), intr_tx.clone());
    let (render_tx, render_rx) = mpsc::channel();
    sq::research(sq.clone(), input.clone(), intr_rx, render_tx.clone());
    ui.start(sq, input, intr_tx, render_tx, render_rx);
}

fn get_async_candidates(sq: Arc<Mutex<sq::Sq>>, ch: mpsc::Sender<()>) {
    let _ = thread::scoped(move|| {
        let mut stdin = std::io::stdin();
        loop {
            let mut buf = String::new();
            match stdin.read_line(&mut buf) {
                Ok(0) => return,
                Ok(_) => {},
                Err(e) => panic!("{}", e),
            }
            let mut sq = sq.lock().unwrap();
            sq.candidates.push(buf);
            ch.send(()).unwrap();
        }
    });
}
