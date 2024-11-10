mod snake;
mod area;
mod apple;
mod gameplay;
use apple::Apple;
use area::{Border, Direction, Position, Seg};
use gameplay::play;
use snake::Snake;
use std::sync::mpsc::{Receiver, Sender};
use std::time::Duration;
use std::{sync::mpsc, thread};

use crossterm::{
    cursor,
    event::{self, Event, KeyCode},
    execute,
    terminal::{self, ClearType},
};
use std::io::{self, Write};

fn main() {
    play();
}


