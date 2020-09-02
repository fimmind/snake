use std::io::stdin;
use std::sync::mpsc;
use std::thread;
use termion::event::Key;
use termion::input::TermRead;

pub struct KeysQueue {
    rx: mpsc::Receiver<Key>,
}

impl KeysQueue {
    pub fn start() -> Self {
        let (tx, rx) = mpsc::channel();
        thread::spawn(move || {
            for key in stdin().keys() {
                if let Err(_) = tx.send(key.unwrap()) {
                    break;
                }
            }
        });
        KeysQueue { rx }
    }

    pub fn pop(&self) -> Option<Key> {
        self.rx.try_recv().ok()
    }
}
