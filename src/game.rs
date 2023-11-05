use crate::{core::board::Board, input::Input};
use macroquad::miniquad::date::now;
use std::boxed::Box;

pub struct Game {
    pub board: Box<Board>,
    pub input: Input,
}

impl Default for Game {
    fn default() -> Self {
        Self {
            board: Box::new(Board::new(now() as usize)),
            input: Input::default(),
        }
    }
}
