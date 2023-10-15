use crate::{input::Input, tetris::board::Board};
use macroquad::miniquad::date::now;
use std::boxed::Box;

pub struct Game {
    pub board: Box<Board>,
    pub input: Input,
}

impl Game {
    pub fn new() -> Self {
        Self {
            board: Box::new(Board::new(now() as usize)),
            input: Input::default(),
        }
    }
}
