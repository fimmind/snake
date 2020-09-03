use ndarray::Array2;
use std::cell::RefCell;
use std::io::{Stdout, Write};
use std::iter;
use std::rc::Rc;
use termion::clear;
use termion::color::{self, Color, Fg};
use termion::cursor;
use termion::raw::RawTerminal;
use termion::screen::AlternateScreen;

pub struct Field {
    cells: Array2<String>,
    shown: bool,
    screen: Rc<RefCell<AlternateScreen<RawTerminal<Stdout>>>>,
}

impl Field {
    pub fn new(
        screen: Rc<RefCell<AlternateScreen<RawTerminal<Stdout>>>>,
        size: (usize, usize),
    ) -> Self {
        Field {
            cells: Array2::default(size),
            shown: false,
            screen,
        }
    }

    pub fn show(&mut self) {
        self.shown = true;

        let mut screen = self.screen.borrow_mut();
        write!(screen, "{}{}", clear::All, cursor::Hide).unwrap();
        screen.flush().unwrap();
        for (xy, s) in self.cells.indexed_iter() {
            print_at_cell(screen.by_ref(), xy, s);
        }
        drop(screen);
        self.draw_borders();
    }

    pub fn hide(&mut self) {
        self.shown = false;
        let mut screen = self.screen.borrow_mut();
        write!(screen, "{}{}", clear::All, cursor::Show).unwrap();
        screen.flush().unwrap();
    }

    fn draw_borders(&mut self) {
        let (size_x, size_y) = self.cells.dim();
        let mut screen = self.screen.borrow_mut();
        write!(screen, "{}", Fg(color::White)).unwrap();

        write!(
            screen,
            "{}┏{}┓",
            cursor::Goto(1, 1),
            iter::repeat('━').take(size_x * 2 + 1).collect::<String>(),
        )
        .unwrap();

        for i in 1..=size_y as u16 {
            write!(
                screen,
                "{}┃{}┃",
                cursor::Goto(1, i + 1),
                cursor::Goto(size_x as u16 * 2 + 3, i + 1),
            )
            .unwrap();
        }

        write!(
            screen,
            "{}┗{}┛",
            cursor::Goto(1, size_y as u16 + 2),
            iter::repeat('━').take(size_x * 2 + 1).collect::<String>(),
        )
        .unwrap();

        write!(screen, "{}", Fg(color::Reset)).unwrap();
        screen.flush().unwrap();
    }

    pub fn set_cell(&mut self, cell: (usize, usize), color: impl Color) {
        let cell_value = format!("{}■{}", Fg(color), Fg(color::Reset));
        if self.shown {
            print_at_cell(self.screen.borrow_mut().by_ref(), cell, &cell_value);
        }
        self.cells[cell] = cell_value;
    }

    pub fn unset_cell(&mut self, cell: (usize, usize)) {
        self.cells[cell] = String::new();
        if self.shown {
            print_at_cell(self.screen.borrow_mut().by_ref(), cell, " ");
        }
    }
}

fn print_at_cell<S: Write>(screen: &mut S, (x, y): (usize, usize), cell_value: &str) {
    let x = x as u16;
    let y = y as u16;
    write!(screen, "{}{}", cursor::Goto(x * 2 + 3, y + 2), cell_value).unwrap();
    screen.flush().unwrap();
}
