use crate::area::Position;
use rand::Rng;
use std::io::{self, Write};

use crossterm::{cursor, execute};

pub struct Apple {
    pub pos: Position
}

impl Apple {
    pub fn next() -> Apple{
        let x = rand::thread_rng().gen_range(1..=39);
        let y = rand::thread_rng().gen_range(1..=14);

        Apple{
            pos : Position {
                x : x,
                y : y
            }
        }
    }

    pub fn print(&self, out: &mut io::Stdout){
        _ = execute!(out, cursor::MoveTo(self.pos.x, self.pos.y));
        _ = writeln!(out, "*");
    }

    pub fn intersect(&self, x: u16, y: u16) -> bool{
        self.pos.x == x && self.pos.y == y
    }
}
