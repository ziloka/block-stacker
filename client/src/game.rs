use std::cell::Cell;

use crate::{drawer::Drawer, input::Input};
use macroquad::miniquad::date::now;
use tetris::{board::Board, consts::Vec2};

pub struct Game<'a> {
    pub board: Board,
    pub drawer: Drawer<'a>,
    pub input: Input,
}

impl<'a> Game<'a> {
    pub fn new(left_top_corner: Vec2, debug: &'a Cell<bool>) -> Self {
        Self {
            board: Board::new(now() as usize),
            drawer: Drawer {
                left_top_corner,
                debug,
            },
            input: Input::default(),
        }
    }
}
