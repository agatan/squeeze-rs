use std::thread;
use std::sync::{mpsc, Arc, Mutex};

pub mod result;
pub mod mode;

pub struct Sq {
    needle: String,
    mode: mode::Mode,
    pub candidates: Vec<String>,
    results: Vec<result::Result>,
    render_trigger: mpsc::Sender<()>,
}

impl Sq {
    pub fn new(mode: mode::Mode, render_trigger: mpsc::Sender<()>) -> Sq {
        Sq {
            needle: String::new(),
            mode: mode,
            candidates: vec![],
            results: vec![],
            render_trigger: render_trigger,
        }
    }

    pub fn push_candidate(&mut self, candidate: String) {
        self.candidates.push(candidate);
    }

    pub fn push_input(&mut self, ch: char) {
        self.needle.push(ch);
    }

    pub fn refine(&mut self) {
        self.results = self.mode.refine_vec(&self.candidates, &self.needle);
        self.render_trigger.send(()).unwrap();
    }

    pub fn needle(&self) -> &str {
        self.needle.as_str()
    }

    pub fn results(&self) -> &[result::Result] {
        &self.results
    }
}
