use crate::{
    input::Input,
    tetris::{board::Board, drawer::Drawer},
};
use macroquad::miniquad::date::now;

pub struct Game<'a> {
    pub board: Board<'a>,
    pub input: Input,
}

impl<'a> Game<'a> {
    pub fn new(
        drawer: &'a dyn Drawer
    ) -> Self {
        Self {
            board: Board::new(drawer, now() as usize),
            input: Input::default(),
        }
    }
}
