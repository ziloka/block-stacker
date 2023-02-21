use macroquad::{
    miniquad::date::now,
    prelude::{is_key_down, is_key_pressed},
};

use crate::{
    settings::Settings,
    tetris::{
        board::Board,
        consts::{vec2, HEIGHT},
    },
};

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
                if board.conflict(&board.active_piece.dots, vec2(0.0, y), true) {
                    y_offset = y - 1.0;
                    break;
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
    }
}
