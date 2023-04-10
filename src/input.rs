use macroquad::{
    miniquad::date::now,
    prelude::{is_key_down, is_key_pressed},
};

use crate::{settings::Settings, tetris::board::Board};

#[derive(Default)]
pub struct Input {
    pub settings: Settings,
    das: f64, // the frames that have passed since the last movement
    arr: f64, // the frames that have passed since the last movement
}

impl Input {
    // handles movement for every frame
    pub fn handle(&mut self, board: &mut Board) {
        let held_down = is_key_down(self.settings.controls.left)
            || is_key_down(self.settings.controls.right)
            || is_key_down(self.settings.controls.soft_drop);

        let is_down = is_key_pressed(self.settings.controls.left)
            || is_key_pressed(self.settings.controls.right)
            || is_key_pressed(self.settings.controls.soft_drop);

        let das = self.settings.handles.das as f64;
        let arr = self.settings.handles.arr as f64;

        if held_down {
            self.das += 1.0;
        } else {
            self.das = 0.0;
        }

        if is_down {
            self.arr += 1.0;
        } else {
            self.arr = 0.0;
        }

        if self.das >= das || self.arr >= arr {
            if is_key_down(self.settings.controls.left) {
                board.move_left();
            }
            if is_key_down(self.settings.controls.right) {
                board.move_right();
            }
            if is_key_down(self.settings.controls.soft_drop) {
                board.soft_drop();
            }
        }

        // https://github.com/JohnnyTurbo/LD43/blob/82de0ac5aa29f6e87d6c5417e0504d6ae7033ef6/Assets/Scripts/PiecesController.cs#L140-L147
        if is_key_pressed(self.settings.controls.rotate_clockwise) {
            board.rotate_tetromino(true, true);
        } else if is_key_pressed(self.settings.controls.rotate_counterclockwise) {
            board.rotate_tetromino(false, true);
        } else if is_key_pressed(self.settings.controls.rotate_180) {
            board.rotate_tetromino(true, true);
            board.rotate_tetromino(true, true);
        } else if is_key_pressed(self.settings.controls.hard_drop) {
            board.hard_drop();
        } else if is_key_pressed(self.settings.controls.hold) {
            board.hold_tetromino();
        } else if is_key_pressed(self.settings.controls.restart) {
            board.restart(now() as usize);
        }
    }
}
