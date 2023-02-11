use std::cell::Cell;

use crate::{
    drawer::Drawer,
    input::Input,
    tetris::{board::Board, consts::Vec2},
};
use macroquad::miniquad::date::now;

pub struct Game<'a> {
    pub board: Board,
    pub drawer: Drawer<'a>,
    pub input: Input,
}

impl<'a> Game<'a> {
    pub fn new(
        left_top_corner: &'a Cell<Vec2>,
        block_size: &'a Cell<f32>,
        debug: &'a Cell<bool>,
    ) -> Self {
        Self {
            board: Board::new(now() as usize),
            drawer: Drawer {
                left_top_corner,
                block_size,
                debug,
            },
            input: Input::default(),
        }
    }
}
