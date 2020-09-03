use std::io::stdin;
use std::sync::mpsc;
use std::thread;
use termion::event::Key;
use termion::input::TermRead;
use std::convert::TryFrom;

pub struct KeysEventsQueue<E> {
    rx: mpsc::Receiver<E>,
}

impl<E: TryFrom<Key> + Send + 'static> KeysEventsQueue<E> {
    pub fn start() -> Self {
        let (tx, rx) = mpsc::channel();
        thread::spawn(move || {
            for key in stdin().keys() {
                if let Ok(event) = E::try_from(key.unwrap()) {
                    if let Err(_) = tx.send(event) {
                        break;
                    }
                }
            }
        });
        KeysEventsQueue { rx }
    }

    pub fn pop(&self) -> Option<E> {
        self.rx.try_recv().ok()
    }
}
