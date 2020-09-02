use ndarray::Array2;
use std::io::{stdout, Stdout, Write};
use std::iter;
use termion::clear;
use termion::color::{self, Color, Fg};
use termion::cursor;
use termion::raw::{IntoRawMode, RawTerminal};
use termion::screen::{AlternateScreen, ToAlternateScreen, ToMainScreen};

pub struct Field {
    cells: Array2<String>,
    shown: bool,
    term: AlternateScreen<RawTerminal<Stdout>>,
}

impl Field {
    pub fn new(size: (usize, usize)) -> Self {
        let mut term = AlternateScreen::from(stdout().into_raw_mode().unwrap());
        write!(term, "{}", ToMainScreen).unwrap();
        term.suspend_raw_mode().unwrap();
        Field {
            cells: Array2::default(size),
            shown: false,
            term,
        }
    }

    pub fn show(&mut self) {
        self.shown = true;

        write!(
            self.term,
            "{}{}{}",
            ToAlternateScreen,
            clear::All,
            cursor::Hide
        )
        .unwrap();
        self.term.flush().unwrap();
        self.term.activate_raw_mode().unwrap();

        self.draw_border();
        for ((x, y), s) in self.cells.indexed_iter() {
            print_at_cell(&mut self.term, (x, y), s);
        }
    }

    pub fn hide(&mut self) {
        self.shown = false;
        write!(self.term, "{}{}", ToMainScreen, cursor::Show).unwrap();
        self.term.flush().unwrap();
        self.term.suspend_raw_mode().unwrap();
    }

    pub fn draw_border(&mut self) {
        let (size_x, size_y) = self.cells.dim();
        write!(self.term, "{}", Fg(color::White)).unwrap();

        write!(
            self.term,
            "{}┏{}┓",
            cursor::Goto(1, 1),
            iter::repeat('━').take(size_x * 2 + 1).collect::<String>(),
        )
        .unwrap();

        for i in 1..=size_y as u16 {
            write!(
                self.term,
                "{}┃{}┃",
                cursor::Goto(1, i + 1),
                cursor::Goto(size_x as u16 * 2 + 3, i + 1),
            )
            .unwrap();
        }

        write!(
            self.term,
            "{}┗{}┛",
            cursor::Goto(1, size_y as u16 + 2),
            iter::repeat('━').take(size_x * 2 + 1).collect::<String>(),
        ).unwrap();

        write!(self.term, "{}", Fg(color::Reset)).unwrap();
    }

    pub fn set_cell(&mut self, cell: (usize, usize), color: impl Color) {
        let cell_value = format!("{}■{}", Fg(color), Fg(color::Reset));
        if self.shown {
            print_at_cell(&mut self.term, cell, &cell_value);
        }
        self.cells[cell] = cell_value;
    }

    pub fn unset_cell(&mut self, cell: (usize, usize)) {
        self.cells[cell] = String::new();
        if self.shown {
            print_at_cell(&mut self.term, cell, " ");
        }
    }
}

fn print_at_cell<S: Write>(screen: &mut S, (x, y): (usize, usize), cell_value: &str) {
    let x = x as u16;
    let y = y as u16;
    write!(screen, "{}{}", cursor::Goto(x * 2 + 3, y + 2), cell_value,).unwrap();
    screen.flush().unwrap();
}
