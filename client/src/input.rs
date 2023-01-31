use std::default;

use macroquad::{
    miniquad::date::now,
    prelude::{is_key_down, is_key_pressed},
};

use tetris::{
    board::Board,
    consts::{vec2, HEIGHT},
};

use crate::settings::Settings;

pub struct Input {
    pub settings: Settings,
    holding: bool,
    frame: f64, // the time the last movement was made
}

impl default::Default for Input {
    fn default() -> Self {
        Self {
            settings: Settings::default(),
            holding: false,
            frame: 0.0,
        }
    }
}

impl Input {
    // handles movement for every frame
    pub fn handle(&mut self, board: &mut Board) {
        self.holding = is_key_down(self.settings.controls.left)
            || is_key_down(self.settings.controls.right)
            || is_key_down(self.settings.controls.soft_drop);

        let das = self.settings.handles.das as f64;
        let arr = self.settings.handles.arr as f64;
        let frame = self.frame;

        // println!("left: {}, right: {}, soft_drop: {}", is_key_down(self.settings.controls.left), is_key_down(self.settings.controls.right), is_key_down(self.settings.controls.soft_drop));

        if (!self.holding && frame % das <= das) || (self.holding && frame % arr <= arr) {
            if is_key_down(self.settings.controls.left)
                && !board.conflict(&board.active_piece.dots, vec2(-1.0, 0.0), true)
            {
                for dot in board.active_piece.dots.iter_mut() {
                    dot.x -= 1.0;
                }
            }
            if is_key_down(self.settings.controls.right)
                && !board.conflict(&board.active_piece.dots, vec2(1.0, 0.0), true)
            {
                for dot in board.active_piece.dots.iter_mut() {
                    dot.x += 1.0;
                }
            }
            if is_key_down(self.settings.controls.soft_drop)
                && !board.conflict(&board.active_piece.dots, vec2(0.0, 1.0), true)
            {
                for dot in board.active_piece.dots.iter_mut() {
                    dot.y += 1.0;
                }
            }
        }

        // https://github.com/JohnnyTurbo/LD43/blob/82de0ac5aa29f6e87d6c5417e0504d6ae7033ef6/Assets/Scripts/PiecesController.cs#L140-L147
        if is_key_pressed(self.settings.controls.rotate_clockwise) {
            board.rotate_tetromino(false, true);
        } else if is_key_pressed(self.settings.controls.rotate_counterclockwise) {
            board.rotate_tetromino(true, true);
        } else if is_key_pressed(self.settings.controls.rotate_180) {
            board.rotate_tetromino(true, true);
            board.rotate_tetromino(true, true);
        } else if is_key_pressed(self.settings.controls.hard_drop) {
            // the hard drop
            let mut y_offset = 0.0;
            for y in 0..HEIGHT as i32 {
                let y = y as f32;
                if !board.conflict(&board.active_piece.dots, vec2(0.0, y), true) {
                    y_offset = y;
                }
            }
            for dot in board.active_piece.dots.iter_mut() {
                dot.y += y_offset;
            }
            board.set_active_tetromino_position();
        } else if is_key_pressed(self.settings.controls.hold) {
            board.hold_tetromino();
        } else if is_key_pressed(self.settings.controls.restart) {
            *board = Board::new(now() as usize);
        }

        // handle line clears
        board.clear_lines();

        // update frame
        self.frame += 1.0;
    }
}
