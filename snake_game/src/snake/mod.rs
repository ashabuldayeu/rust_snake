use crate::area::{Direction, Position, Seg};
use std::io::{self, Write};

use crossterm::{cursor, execute};

const START_POSITION_X: u16 = 5;
const START_POSITION_Y: u16 = 5;

pub struct Snake {
    pub head: Seg,
    pub tail: Vec<Seg>
}

impl Snake {
    pub fn grow(&mut self){
        let lst = self.tail.last().unwrap();
        self.tail.push(Seg{
            dir : lst.dir,
            pos : Position {
                x : lst.pos.x,
                y : lst.pos.y
            }
        });
    }
    pub fn print_snake(&self, out: &mut io::Stdout) {
        // snake.head.print(out);
        for seg in self.tail.iter() {
            seg.print(out);
        }
    }
    pub fn move_forward(&mut self) {
        let head_next = next_pos(self.head.pos, self.head.dir);
        // let tail_fst = self.head;
        let mut prev_seg = self.tail.first().unwrap().clone();
        let fst_tail = self.tail.first_mut().unwrap();

        fst_tail.pos = self.head.pos;
        fst_tail.dir = self.head.dir;

        self.head.pos = head_next;

        for seg in self.tail.iter_mut().skip(1) {
            let tmp = seg.clone();
            seg.pos = prev_seg.pos;
            seg.dir = prev_seg.dir;

            prev_seg = tmp;
        }
    }
    pub fn set_direction(&mut self, dir: Direction) {
        self.head.dir = dir;
    }
    pub fn new() -> Snake {
        let mut snake = Snake {
            head: Seg {
                pos: Position {
                    x: START_POSITION_X,
                    y: START_POSITION_Y,
                },
                dir: Direction::Top,
            },
            score: 0,
            tail: Vec::new(),
        };
    
        snake.tail.push(Seg {
            pos: Position {
                x: snake.head.pos.x,
                y: snake.head.pos.y - 1,
            },
            dir: snake.head.dir,
        });
    
        snake.tail.push(Seg {
            pos: Position {
                x: snake.head.pos.x,
                y: snake.head.pos.y - 2,
            },
            dir: snake.head.dir,
        });
    
        snake
    }
}
pub fn next_pos(pos: Position, dir: Direction) -> Position {
    match dir {
        Direction::Bot => Position {
            x: pos.x,
            y: pos.y + 1,
        },
        Direction::Top => Position {
            x: pos.x,
            y: pos.y - 1,
        },
        Direction::Left => Position {
            x: pos.x - 1,
            y: pos.y,
        },
        Direction::Right => Position {
            x: pos.x + 1,
            y: pos.y,
        },
    }
}