use std::io::{self, Write};

use crossterm::{cursor, execute};

#[derive(Clone, Copy, PartialEq)]
pub enum Direction {
    Top ,
    Bot,
    Left,
    Right,
}
impl Direction {
    pub fn is_opossite(self, d: Direction) -> bool{
        match  d {
            Direction::Bot => self == Direction::Top,
            Direction::Top => self == Direction::Bot,
            Direction::Right => self == Direction::Left,
            Direction::Left => self == Direction::Right,
        }
    }
}
#[derive(Clone, Copy)]
pub struct Position {
    pub x: u16,
    pub y: u16,
}

pub struct Border {
    pub x_from: u16,
    pub x_to: u16,
    pub y_from: u16,
    pub y_to: u16,
}

impl Border {
    pub fn is_inside(&self, x: u16, y: u16) -> bool {
        self.intersect_x(x) && self.intersect_y(y)
    }

    fn intersect_x(&self, x: u16) -> bool {
        self.x_from < x && x < self.x_to
    }

    fn intersect_y(&self, y: u16) -> bool {
        self.y_from < y && y < self.y_to
    }
    pub fn new() -> Border{
        Border {
            x_from: 0,
            x_to: 40,
            y_from: 0,
            y_to: 15,
        }
    }
    pub fn print(&self, out: &mut io::Stdout) {
        for t in self.x_from ..self.x_to{
            execute!(out, cursor::MoveTo(t, self.x_from));
            writeln!(out, "*");
        }
        for b in self.x_from ..self.x_to{
            execute!(out, cursor::MoveTo(b, self.y_to));
            writeln!(out, "*");
        }

        for l in self.y_from ..self.y_to{
            execute!(out, cursor::MoveTo(self.y_from, l));
            writeln!(out, "*");
        }

        for r in self.y_from ..self.y_to{
            execute!(out, cursor::MoveTo(self.x_to, r));
            writeln!(out, "*");
        }
    }
}

#[derive(Clone, Copy)]
pub struct Seg {
    pub pos: Position,
    pub dir: Direction,
}

impl Seg {
    pub fn print(&self, out: &mut io::Stdout) {
        execute!(out, cursor::MoveTo(self.pos.x, self.pos.y));
        writeln!(out, "*");
    }
}