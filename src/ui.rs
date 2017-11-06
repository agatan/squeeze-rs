use std::sync::{Arc, Mutex};
use std::sync::mpsc::{Sender, Receiver};
use sq::Sq;
use sq::result;

use termfest::{Termfest, Event, Cell, ScreenLock};
use termfest::attr::{Color, Attribute};
use termfest::key::*;

pub struct Ui {
    sq: Arc<Mutex<Sq>>,
    width: usize,
    height: usize,
    term: Termfest,
}

impl Ui {
    pub fn new(sq: Arc<Mutex<Sq>>, term: Termfest) -> Ui {
        let (width, height) = term.lock_screen().size();
        Ui {
            sq,
            width,
            height,
            term,
        }
    }

    pub fn event_handler(&self, ev: Event) -> Option<String> {
        match ev {
            Event::Char(c) => {
                self.sq.lock().unwrap().push_input(c);
            }
            Event::Key(ESC) => return Some(String::new()),
            _ => {}
        }
        return None;
    }

    pub fn render(&self) {
        let mut screen = self.term.lock_screen();
        screen.clear();
        for (idx, r) in self.sq.lock().unwrap().results().iter().enumerate() {
            self.show_result(&mut screen, r, idx + 1, false);
        }
        self.show_prompt(&mut screen);
    }

    fn show_prompt(&self, screen: &mut ScreenLock) {
        let sq = self.sq.lock().unwrap();
        for x in 0..self.width {
            screen.put_cell(x, 0, Cell::new(' '));
        }
        let prompt = "> ";
        screen.print(0, 0, prompt, Attribute::default());
        screen.print(prompt.len(), 0, sq.needle(), Attribute::default());
        screen.move_cursor(prompt.len() + sq.needle().len(), 0);
    }

    fn show_result(&self,
                   screen: &mut ScreenLock,
                   result: &result::Result,
                   y: usize,
                   selected: bool) {
        if y >= self.height {
            return;
        }
        let bg = if selected {
            Color::Green
        } else {
            Color::Default
        };
        for x in 0..self.width {
            screen.put_cell(x, y + 1, Cell::new(' ').bg(bg));
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
            screen.put_cell(idx, y + 1, Cell::new(c).fg(fg).bg(bg));
        }
    }
}
