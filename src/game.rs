use std::boxed::Box;
use crate::{
    input::Input,
    tetris::{board::Board, drawer::Drawer},
};
use macroquad::miniquad::date::now;

pub struct Game<'a> {
    pub board: Box<Board<'a>>,
    pub input: Input<'a>,
}

impl<'a> Game<'a> {
    pub fn new(drawer: &'a dyn Drawer) -> Self {
        Self {
            board: Box::new(Board::new(drawer, now() as usize)),
            input: Input::default(),
        }
    }
}
