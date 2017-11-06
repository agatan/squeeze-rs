use std::sync::{Arc, Mutex};
use std::sync::mpsc::{Sender, Receiver};
use sq::Sq;
use sq::result;

use termfest::{Termfest, Event, Cell};
use termfest::attr::{Color, Attribute};

pub struct Ui {
    width: usize,
    height: usize,
    term: Termfest,
    events: Receiver<Event>,
}

impl Ui {
    pub fn new() -> Ui {
        let (term, events) = Termfest::hold().expect("failed to initialize terminal");
        let (width, height) = term.lock_screen().size();
        Ui {
            width,
            height,
            term,
            events,
        }
    }

    pub fn start(self,
                 sq: Arc<Mutex<Sq>>,
                 input: Arc<Mutex<String>>,
                 intr_tx: Sender<()>,
                 render_tx: Sender<usize>,
                 render_rx: Receiver<usize>) {
        loop {
            for ev in self.events.iter() {
                match ev {
                    Event::Char(c) => {
                        input.lock().unwrap().push(c);
                        intr_tx.send(()).unwrap();
                    }
                    _ => {}
                }
                render_tx.send(0).unwrap();
            }
        }
    }

    pub fn clear(&self) {
        self.term.lock_screen().clear();
    }


    pub fn show_prompt(&self, input: &str) {
        let mut screen = self.term.lock_screen();
        for x in 0..self.width {
            screen.put_cell(x,
                            0,
                            Cell {
                                ch: ' ',
                                attribute: Attribute::default(),
                            });
        }
        let prompt = "> ";
        screen.print(0, 0, prompt, Attribute::default());
        screen.print(prompt.len(), 0, input, Attribute::default());
        screen.move_cursor(prompt.len() + input.len(), 0);
    }

    fn show_result(&self, result: &result::Result, y: usize, selected: bool) {
        if y >= self.height {
            return;
        }
        let bg = if selected {
            Color::Green
        } else {
            Color::Default
        };
        let mut screen = self.term.lock_screen();
        for x in 0..self.width {
            screen.put_cell(x,
                            y + 1,
                            Cell {
                                ch: ' ',
                                attribute: Attribute {
                                    bg: bg,
                                    ..Attribute::default()
                                },
                            });
        }
        if result.matches.is_empty() {
            screen.print(0,
                         y + 1,
                         &result.string,
                         Attribute {
                             bg: bg,
                             ..Attribute::default()
                         });
            return;
        }
        for (idx, c) in result.string.chars().enumerate() {
            let fg = if result.matches.contains(&idx) {
                Color::Red
            } else {
                Color::Default
            };
            screen.put_cell(idx,
                            y + 1,
                            Cell {
                                ch: c,
                                attribute: Attribute {
                                    fg: fg,
                                    bg: bg,
                                    ..Attribute::default()
                                },
                            });
        }
    }
}
