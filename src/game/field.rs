use ndarray::Array2;
use std::io::{prelude::*, stdout, Stdout, Write};
use termion::clear;
use termion::color::{self, Color};
use termion::cursor;
use termion::raw::{IntoRawMode, RawTerminal};

pub struct Field {
    cells: Array2<String>,
    drawn: bool,
    term: RawTerminal<Stdout>,
}

impl Field {
    pub fn new(size: (usize, usize)) -> Self {
        let term = stdout().into_raw_mode().unwrap();
        term.suspend_raw_mode().unwrap();
        Field {
            cells: Array2::default(size),
            drawn: false,
            term,
        }
    }

    pub fn draw(&mut self) {
        self.drawn = true;
        self.term.activate_raw_mode().unwrap();

        write!(self.term, "{}{}", cursor::Hide, clear::All).unwrap();
        self.term.flush().unwrap();
        for ((x, y), s) in self.cells.indexed_iter() {
            print_at_cell(&mut self.term, (x, y), s);
        }
    }

    pub fn hide(&mut self) {
        self.drawn = false;
        write!(self.term, "{}{}{}", clear::All, cursor::Show, cursor::Goto(1, 1)).unwrap();
        self.term.flush().unwrap();
        self.term.suspend_raw_mode().unwrap();
    }

    pub fn set_cell(&mut self, cell: (usize, usize), color: impl Color) {
        let cell_value = format!("{}■{}", color::Fg(color), color::Fg(color::Reset));
        if self.drawn {
           print_at_cell(&mut self.term, cell, &cell_value);
        }
        self.cells[cell] = cell_value;
    }

    pub fn unset_cell(&mut self, cell: (usize, usize)) {
        self.cells[cell] = String::new();
        if self.drawn {
            print_at_cell(&mut self.term, cell, " ");
        }
    }

}

fn print_at_cell<S: Write>(screen: &mut S, (x, y): (usize, usize), cell_value: &str) {
    let x = x as u16;
    let y = y as u16;
    write!(
        screen,
        "{}{}",
        cursor::Goto(x * 2 + 1, y + 1),
        cell_value,
    ).unwrap();
    screen.flush().unwrap();
}
