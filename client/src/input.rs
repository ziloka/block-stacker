use std::default;

use macroquad::{
    prelude::{is_key_down, is_key_pressed},
    time::get_time,
};

use tetris::{board::Board, consts::{HEIGHT, vec2}};

use crate::settings::Settings;

pub struct Input {
    pub settings: Settings,
    holding: bool,
    time: f64, // the time the last movement was made
}

impl default::Default for Input {
    fn default() -> Self {
        Self {
            holding: false,
            time: get_time() * 1000.0,
            settings: Settings::default()
        }
    }
}

impl Input {
    // handles movement
    pub fn handle(&mut self, board: &mut Board) {

        self.holding = is_key_pressed(self.settings.controls.left)
            || is_key_pressed(self.settings.controls.right)
            || is_key_pressed(self.settings.controls.soft_drop);

        if !self.holding
            && (self.time % self.settings.handles.das as f64) < self.settings.handles.das as f64
            || (self.holding
                && (self.time % self.settings.handles.arr as f64)
                    < self.settings.handles.arr as f64)
        {
            if is_key_down(self.settings.controls.left) && !board.conflict(vec2(-1.0, 0.0)) {
                for dot in board.active_piece.dots.iter_mut() {
                    dot.x -= 1.0;
                }
            } else if is_key_down(self.settings.controls.right) && !board.conflict(vec2(1.0, 0.0))
            {
                for dot in board.active_piece.dots.iter_mut() {
                    dot.x += 1.0;
                }
            } else if is_key_down(self.settings.controls.soft_drop)
                && !board.conflict(vec2(0.0, 1.0))
            {
                for dot in board.active_piece.dots.iter_mut() {
                    dot.y += 1.0;
                }
            }
        }

        // https://github.com/JohnnyTurbo/LD43/blob/82de0ac5aa29f6e87d6c5417e0504d6ae7033ef6/Assets/Scripts/PiecesController.cs#L140-L147
        if is_key_pressed(self.settings.controls.rotate_clockwise) {
            board.rotate_tetrimino(true, true); // rotate clockwise
        } else if is_key_pressed(self.settings.controls.rotate_counterclockwise) {
            board.rotate_tetrimino(false, true); // rotate clockwise
        } else if is_key_pressed(self.settings.controls.hard_drop) {
            // the hard drop
            let mut y_offset = 0.0;
            for y in 0..HEIGHT as i32 {
                let y = y as f32;
                if !board.conflict(vec2(0.0, y)) {
                    y_offset = y;
                }
            }
            for dot in board.active_piece.dots.iter_mut() {
                dot.y += y_offset;
            }
            board.set_active_tetrimino_position();
        } else if is_key_pressed(self.settings.controls.hold) {
            board.hold_tetrimino();
        }

        // handle line clears
        board.clear_lines();

        // update time
        self.time = get_time() * 1000.0;
    }
}
