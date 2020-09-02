mod field;

use itertools::iproduct;
use rand::{rngs::ThreadRng, seq::IteratorRandom, thread_rng};
use std::collections::{HashSet, VecDeque};
use std::io::{prelude::*, stdin, stdout};
use std::process;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, SystemTime};
use termion::color;
use termion::event::Key;
use termion::input::TermRead;

use field::Field;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Direction {
    Right,
    Up,
    Left,
    Down,
}

impl Direction {
    fn opposite(self) -> Self {
        use Direction::*;
        match self {
            Right => Left,
            Up => Down,
            Left => Right,
            Down => Up,
        }
    }

    fn move_point(
        self,
        (x_size, y_size): (usize, usize),
        (mut x, mut y): (usize, usize),
    ) -> (usize, usize) {
        x += x_size;
        y += y_size;
        match self {
            Direction::Right => x += 1,
            Direction::Up => y -= 1,
            Direction::Left => x -= 1,
            Direction::Down => y += 1,
        }
        x %= x_size;
        y %= y_size;

        (x, y)
    }
}

#[derive(Debug, Copy, Clone)]
enum Event {
    Move(Direction),
    Quit,
}

#[derive(Debug, Clone)]
struct EventsQueue {
    queue: Arc<Mutex<VecDeque<Event>>>,
}

impl EventsQueue {
    fn start() -> Self {
        let events_queue = Arc::new(Mutex::new(VecDeque::new()));
        let ret = events_queue.clone();

        thread::spawn(move || {
            for event in stdin().keys() {
                if let Ok(key) = event {
                    events_queue.lock().unwrap().push_back(match key {
                        Key::Char('h') | Key::Left => Event::Move(Direction::Left),
                        Key::Char('j') | Key::Down => Event::Move(Direction::Down),
                        Key::Char('k') | Key::Up => Event::Move(Direction::Up),
                        Key::Char('l') | Key::Right => Event::Move(Direction::Right),
                        Key::Char('q') | Key::Esc | Key::Ctrl('c') => Event::Quit,
                        _ => continue,
                    })
                }
            }
        });

        EventsQueue { queue: ret }
    }

    fn pop(&self) -> Option<Event> {
        self.queue.lock().unwrap().pop_front()
    }
}

pub struct Game {
    size: (usize, usize),
    snake: VecDeque<(usize, usize)>,
    empty_cells: HashSet<(usize, usize)>,
    snake_direction: Direction,
    blocked_direction: Direction,
    food: (usize, usize),
    rng: ThreadRng,
    field: Field,
}

impl Game {
    pub fn new(size: (usize, usize)) -> Self {
        let mid_x = size.0 / 2;
        let mid_y = size.1 / 2;
        let mut game = Game {
            size,
            snake: vec![(mid_x, mid_y), (mid_x, mid_y + 1)].into(),
            empty_cells: iproduct!(0..size.0, 0..size.1).collect(),
            snake_direction: Direction::Up,
            blocked_direction: Direction::Down,
            food: (0, 0),
            rng: thread_rng(),
            field: Field::new(size),
        };
        for &cell in game.snake.iter() {
            game.field.set_cell(cell, color::White);
            game.empty_cells.remove(&cell);
        }
        game.gen_food();
        game
    }

    fn set_direction(&mut self, dir: Direction) -> bool {
        if self.blocked_direction != dir {
            self.snake_direction = dir;
            return true;
        }
        false
    }

    fn gen_food(&mut self) {
        self.food = *self.empty_cells.iter().choose(&mut self.rng).unwrap();
        self.field.set_cell(self.food, color::Red);
    }

    fn snake_push_head(&mut self, cell: (usize, usize)) {
        self.snake.push_front(cell);
        self.field.set_cell(cell, color::White);
        self.empty_cells.remove(&cell);
    }

    fn snake_pop_tail(&mut self) {
        let tail = self.snake.pop_back().unwrap();
        self.field.unset_cell(tail);
        self.empty_cells.insert(tail);
    }

    fn make_step(&mut self) {
        let next_step = self
            .snake_direction
            .move_point(self.size, *self.snake.front().unwrap());

        if self.snake.iter().any(|piece| piece == &next_step) {
            self.stop("You died");
        }

        self.snake_push_head(next_step);

        if next_step == self.food {
            self.gen_food();
        } else {
            self.snake_pop_tail();
        }

        if self.snake.len() == self.size.0 * self.size.1 {
            self.stop("You won!");
        }

        self.blocked_direction = self.snake_direction.opposite();
    }

    pub fn start(mut self, move_delay: u64) -> ! {
        self.field.draw();
        let events_queue = EventsQueue::start();
        let mut next_step_time = SystemTime::now();
        loop {
            if let Some(event) = events_queue.pop() {
                match event {
                    Event::Move(dir) => {
                        if self.set_direction(dir) {
                            self.make_step();
                            next_step_time = SystemTime::now() + Duration::from_millis(move_delay);
                        }
                    }
                    Event::Quit => self.stop("Goodbye"),
                };
            } else if next_step_time <= SystemTime::now() {
                self.make_step();
                next_step_time = SystemTime::now() + Duration::from_millis(move_delay);
            }
        }
    }

    fn stop(&mut self, message: &str) -> ! {
        self.field.hide();
        print!("{}", message);
        stdout().flush().unwrap();
        process::exit(0);
    }
}
