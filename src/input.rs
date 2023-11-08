use macroquad::{
    miniquad::date::now,
    prelude::{is_key_down, is_key_pressed},
};

use crate::{
    core::{board::Board, consts::vec2},
    extensions::history::History,
    settings::Settings,
};

#[derive(Default)]
pub struct Input {
    pub settings: Settings,
    das: f64, // the frames that have passed since the last movement
    arr: f64, // the frames that have passed since the last movement
    last_checked: f64,
    history: History,
}

impl Input {
    // handles movement for every frame
    pub fn handle(&mut self, board: &mut Box<Board>) {
        let held_down = is_key_down(self.settings.controls.left)
            || is_key_down(self.settings.controls.right)
            || is_key_down(self.settings.controls.soft_drop);

        let das = self.settings.handles.das as f64;
        let arr = self.settings.handles.arr as f64;

        if held_down {
            self.das = (now() - self.last_checked) * 1000.0;
        } else {
            self.das = 0.;
            self.arr = 0.;
            self.last_checked = now();
        }

        if self.das >= das {
            self.arr = (now() - self.last_checked) * 1000.0;
        }

        let is_down = is_key_pressed(self.settings.controls.left)
            || is_key_pressed(self.settings.controls.right)
            || is_key_pressed(self.settings.controls.soft_drop);

        if is_down || (self.das >= das && self.arr >= arr) {
            if is_key_down(self.settings.controls.left)
                && !board.conflict(&board.active_piece.dots, vec2(-1.0, 0.0), true)
            {
                board.move_left();
                self.history.add_state(board.clone());
            }
            if is_key_down(self.settings.controls.right)
                && !board.conflict(&board.active_piece.dots, vec2(1.0, 0.0), true)
            {
                board.move_right();
                self.history.add_state(board.clone());
            }
            if is_key_down(self.settings.controls.soft_drop)
                && !board.conflict(&board.active_piece.dots, vec2(0.0, -1.0), true)
            {
                board.soft_drop();
            }
            self.arr = 0.;
        }

        if is_key_pressed(self.settings.controls.rotate_clockwise) {
            if board.rotate_tetromino_90(true, true) {
                self.history.add_state(board.clone());
            }
        } else if is_key_pressed(self.settings.controls.rotate_counterclockwise) {
            if board.rotate_tetromino_90(false, true) {
                self.history.add_state(board.clone());
            }
        } else if is_key_pressed(self.settings.controls.rotate_180) {
            board.rotate_tetromino_180(true);
            self.history.add_state(board.clone());
        } else if is_key_pressed(self.settings.controls.hard_drop) {
            board.hard_drop();
            self.history.add_state(board.clone());
        } else if is_key_pressed(self.settings.controls.hold) {
            board.hold_tetromino();
            self.history.add_state(board.clone());
        } else if is_key_pressed(self.settings.controls.restart) {
            board.restart(now() as usize);
            self.history.add_state(board.clone());
        } else if is_key_pressed(self.settings.controls.undo) {
            if let Some(new_board) = self.history.undo() {
                *board = new_board;
            }
        }
        if is_key_pressed(self.settings.controls.redo) {
            if let Some(new_board) = self.history.redo() {
                *board = new_board;
            }
        }
    }
}
