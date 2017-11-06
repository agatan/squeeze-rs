use std;
use std::thread;
use std::sync::{mpsc, Arc, Mutex};

pub mod result;
pub mod mode;

pub struct Sq {
    mode: mode::Mode,
    pub candidates: Vec<String>,
    results: Vec<result::Result>,
}

impl Sq {
    pub fn new(mode: mode::Mode) -> Sq {
        Sq {
            mode: mode,
            candidates: vec![],
            results: vec![],
        }
    }

    pub fn refine(&mut self, needle: &str) {
        self.results = self.mode.refine_vec(&self.candidates, needle);
    }
}

pub fn research(sq: Arc<Mutex<Sq>>,
                needle: Arc<Mutex<String>>,
                interrupt: mpsc::Receiver<()>,
                sx: mpsc::Sender<usize>) {
    sq.lock().unwrap().results.clear();
    let mut idx = 0;
    loop {
        if let Ok(_) = interrupt.try_recv() {
            thread::spawn(move || { research(sq, needle, interrupt, sx); });
            return;
        }
        let mut sq = sq.lock().unwrap();
        if idx >= sq.candidates.len() {
            return;
        }
        if let Some(r) = sq.mode
               .refine_string(&sq.candidates[idx], &needle.lock().unwrap()) {
            sq.results.push(r);
            sx.send(idx).unwrap();
        }
        idx += 1;
    }
}
