mod field;
mod keys_queue;

use itertools::iproduct;
use rand::{rngs::ThreadRng, seq::IteratorRandom, thread_rng};
use std::collections::{HashSet, VecDeque};
use std::process;
use std::time::{Duration, SystemTime};
use termion::color;
use termion::event::Key;

use field::Field;
use keys_queue::KeysQueue;

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
}

#[derive(Debug, Copy, Clone)]
enum Event {
    Move(Direction),
    Quit,
}

impl Event {
    fn from_key(key: Key) -> Option<Self> {
        Some(match key {
            Key::Char('h') | Key::Char('a') | Key::Left => Event::Move(Direction::Left),
            Key::Char('j') | Key::Char('s') | Key::Down => Event::Move(Direction::Down),
            Key::Char('k') | Key::Char('w') | Key::Up => Event::Move(Direction::Up),
            Key::Char('l') | Key::Char('d') | Key::Right => Event::Move(Direction::Right),
            Key::Char('q') | Key::Esc | Key::Ctrl('c') => Event::Quit,
            _ => return None,
        })
    }
}

pub struct Game {
    size: (usize, usize),
    snake: VecDeque<(usize, usize)>,
    empty_cells: HashSet<(usize, usize)>,
    snake_direction: Direction,
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

    fn moved_point(&self, (mut x, mut y): (usize, usize), dir: Direction) -> (usize, usize) {
        x += self.size.0;
        y += self.size.1;
        match dir {
            Direction::Right => x += 1,
            Direction::Up => y -= 1,
            Direction::Left => x -= 1,
            Direction::Down => y += 1,
        }
        x %= self.size.0;
        y %= self.size.1;

        (x, y)
    }

    fn gen_food(&mut self) {
        self.food = *self.empty_cells.iter().choose(&mut self.rng).unwrap();
        self.field.set_cell(self.food, color::Red);
    }

    fn push_snake_head(&mut self, cell: (usize, usize)) {
        self.snake.push_front(cell);
        self.field.set_cell(cell, color::White);
        self.empty_cells.remove(&cell);
    }

    fn pop_snake_tail(&mut self) {
        let tail = self.snake.pop_back().unwrap();
        self.field.unset_cell(tail);
        self.empty_cells.insert(tail);
    }

    fn make_step(&mut self, dir: Direction) -> bool {
        if dir == self.snake_direction.opposite() {
            return false;
        }
        self.snake_direction = dir;
        let next_step = self.moved_point(*self.snake.front().unwrap(), dir);

        if !(next_step == self.food) {
            self.pop_snake_tail();
        }
        for cell in self.snake.iter() {
            if cell == &next_step {
                self.stop("You died");
            }
        }
        self.push_snake_head(next_step);

        if next_step == self.food {
            self.gen_food();
        }

        if self.empty_cells.is_empty() {
            self.stop("You won!");
        }
        true
    }

    pub fn start(mut self, move_delay: u64) -> ! {
        self.field.show();
        let keys_queue = KeysQueue::start();
        let mut next_step_time = SystemTime::now();
        let get_next_step_time = || SystemTime::now() + Duration::from_millis(move_delay);
        loop {
            while let Some(key) = keys_queue.pop() {
                Event::from_key(key).map(|event| {
                    match event {
                        Event::Move(dir) => {
                            if self.make_step(dir) {
                                next_step_time = get_next_step_time();
                            }
                        }
                        Event::Quit => self.stop(""),
                    };
                });
            }
            if next_step_time <= SystemTime::now() {
                if self.make_step(self.snake_direction) {
                    next_step_time = get_next_step_time();
                }
            }
        }
    }

    fn stop(&mut self, message: &str) -> ! {
        self.field.hide();
        if !message.is_empty() {
            println!("{}", message);
        }
        process::exit(0);
    }
}
