use std::{default, io, ops::ControlFlow, time::Duration};

use crossterm::{event::{self, Event, KeyCode}, execute, terminal::{self, ClearType}};

use crate::{apple::Apple, area::{Border, Direction}, snake::{self, Snake}};

pub fn play(){
    let mut snake = Snake::new();
    let border = Border::new();
    let mut out = io::stdout();
    let mut apple = Apple::next();
    let mut prev_dir :Direction = Direction::Top;
    
    while border.is_inside(snake.head.pos.x, snake.head.pos.y)  {
        match read_input(&mut prev_dir) {
            Ok(requested_direction) => {
                if requested_direction != prev_dir && !requested_direction.is_opossite(prev_dir){
                    snake.set_direction(requested_direction);
                    prev_dir = requested_direction;
                }
                snake.move_forward();
                _ = execute!(out, terminal::Clear(ClearType::All));

                if apple.intersect(snake.head.pos.x, snake.head.pos.y){
                    snake.grow();
                    apple = Apple::next();
                }

                border.print(&mut out);
                snake.print_snake(&mut out);
                apple.print(&mut out);
            },
            Err(_) => {
                break;
            }
        };
        
    } 
}

fn read_input(prev_dir: &Direction) -> Result<Direction, i32> {
    match event::poll(Duration::from_millis(200)) {
        Ok(is_read) => {
            if is_read {
                match event::read() {
                    Ok(Event::Key(key_event)) 
                        => Ok(match_to_direction(&key_event.code, &*prev_dir)),
                    _   => Err(1)
                }
            }
            else {
                Ok(*prev_dir)
            }
        }
        Err(_x) => Err(1),
    }
}

pub fn match_to_direction(key: &KeyCode, default: &Direction) -> Direction{
    match *key {
        KeyCode::Down => Direction::Bot,
        KeyCode::Up => Direction::Top,
        KeyCode::Left => Direction::Left,
        KeyCode::Right => Direction::Right,
        _ => *default
    }
}