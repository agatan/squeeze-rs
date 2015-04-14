extern crate rustbox;
use std;
use std::sync::{ Arc, Mutex };
use std::sync::mpsc::{Sender, Receiver};
use std::thread;
use sq;
use sq::result;

use rustbox::{RustBox, Color, Key};

pub struct Ui {
    width: usize,
    height: usize,
    rustbox: RustBox,
}

impl Ui {
    pub fn new() -> Ui {
        let rustbox = match RustBox::init(std::default::Default::default()) {
            Ok(r) => r,
            Err(e) => panic!("{}", e),
        };
        Ui{ width: rustbox.width(), height: rustbox.height(), rustbox: rustbox }
    }

    pub fn start(self, sq: Arc<Mutex<sq::Sq>>, input: Arc<Mutex<String>>,
                 intr_tx: Sender<()>, render_tx: Sender<usize>, render_rx: Receiver<usize>) {
        let _ = thread::scoped(move|| {
            // rendering thread
            loop {
                if let Ok(h) = render_rx.recv() {
                    
                } else {
                    panic!("rendering thread: recv error");
                }
            }
        });
        loop {
            if let Ok(rustbox::Event::KeyEvent(Some(key))) = self.rustbox.poll_event(false) {
                match key {
                    Key::Char(c) => {
                        input.lock().unwrap().push(c);
                        intr_tx.send(()).unwrap();
                    },
                    _ => {},
                }
            } else {
                render_tx.send(0).unwrap();
            }
        }
    }

    pub fn clear(&self) {
        self.rustbox.clear();
    }


    pub fn show_prompt(&self, input: &str) {
        for x in 0..self.width {
            self.rustbox.print_char(x, 0, rustbox::RB_NORMAL, Color::Default, Color::Default, ' ');
        }
        let prompt = "> ";
        self.rustbox.print(0, 0, rustbox::RB_NORMAL, Color::Default, Color::Default, prompt);
        self.rustbox.print(prompt.len(), 0, rustbox::RB_NORMAL, Color::Default, Color::Default, input);
        self.rustbox.set_cursor((prompt.len() + input.len()) as isize, 0);
        self.rustbox.present();
    }

    fn show_result(&self, result: &result::Result, y: usize, selected: bool) {
        if y >= self.height { return }
        let bg = if selected { Color::Green } else { Color::Default };
        for x in 0..self.width {
            self.rustbox.print_char(x, y+1, rustbox::RB_NORMAL, Color::Default, bg, ' ');
        }
        if result.matches.is_empty() {
            self.rustbox.print(0, y+1, rustbox::RB_NORMAL, Color::Default, bg, &result.string);
            return;
        }
        for (ref idx, c) in result.string.chars().enumerate() {
            let fg = if result.matches.contains(idx) { Color::Red } else { Color::Default };
            self.rustbox.print_char(*idx, y+1, rustbox::RB_NORMAL, fg, bg, c);
        }
        self.rustbox.present();
    }
}
