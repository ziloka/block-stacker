use std::ops::{Add, Sub};

use macroquad::{
    prelude::{draw_rectangle, is_key_down, is_key_pressed, vec2, Color, Vec2, GRAY},
    time::get_time,
};

use crate::consts::{Piece, Tetrimino, GameState, BLOCK_SIZE, HEIGHT, TETRIMINO_TYPES, WIDTH};
use crate::settings::Settings;

pub struct Board {
    pub left_top_corner: Vec2,
    pub right_bottom_corner: Vec2,
    pub time: f64,
    pub active_piece: Piece,
    pub game_state: GameState,
    pub settings: Settings,
    positions: [[Option<Color>; WIDTH as usize]; HEIGHT as usize],
}

impl Board {
    pub fn new(left_top_corner: Vec2, right_bottom_corner: Vec2) -> Self {
        const CELL_INIT: Option<Color> = None;
        const ROW_INIT: [Option<Color>; WIDTH as usize] = [CELL_INIT; WIDTH as usize];
        let tetrimino_type =
            TETRIMINO_TYPES[macroquad::rand::gen_range::<usize>(0, TETRIMINO_TYPES.len())];
        Self {
            left_top_corner: left_top_corner,
            right_bottom_corner: right_bottom_corner,
            time: get_time() * 1000.0,
            active_piece: Piece {
                tetrimino: tetrimino_type,
                dots: tetrimino_type
                    .get_structure()
                    .into_iter()
                    .map(|pos| {
                        vec2(
                            left_top_corner.x + (WIDTH / 2.0 * BLOCK_SIZE) + pos.x * BLOCK_SIZE,
                            left_top_corner.y + pos.y * BLOCK_SIZE,
                        )
                    })
                    .collect(),
                rotation_index: 0,
            },
            game_state: GameState::Playing,
            settings: Settings::default(),
            // https://stackoverflow.com/a/53930630
            positions: [ROW_INIT; HEIGHT as usize],
        }
    }

    pub fn handle_movement(&mut self) {
        let current_time = macroquad::time::get_time() * 1000.0; // time in miliseconds since the start of the program
                                                                 // https://www.reddit.com/r/Tetris/comments/frbii6/comment/fphx9ml/?utm_source=share&utm_medium=web2x&context=3
        if (is_key_down(self.settings.controls.left) || is_key_down(self.settings.controls.right) || is_key_down(self.settings.controls.soft_drop))
            && (current_time - self.time > self.settings.handles.das as f64
                || ((!is_key_pressed(self.settings.controls.left)
                    || !is_key_pressed(self.settings.controls.right)
                    || !is_key_pressed(self.settings.controls.soft_drop))
                    && current_time - self.time > self.settings.handles.arr as f64))
        {
            if is_key_down(self.settings.controls.left) && !self.conflict(vec2(-BLOCK_SIZE, 0.0)) {
                for dot in self.active_piece.dots.iter_mut() {
                    dot.x -= BLOCK_SIZE;
                }
            } else if is_key_down(self.settings.controls.right) && !self.conflict(vec2(BLOCK_SIZE, 0.0)) {
                for dot in self.active_piece.dots.iter_mut() {
                    dot.x += BLOCK_SIZE;
                }
            } else if is_key_down(self.settings.controls.soft_drop) && !self.conflict(vec2(0.0, BLOCK_SIZE)) {
                for dot in self.active_piece.dots.iter_mut() {
                    dot.y += BLOCK_SIZE;
                }
            }
            self.time = current_time;
        }

        // https://github.com/JohnnyTurbo/LD43/blob/82de0ac5aa29f6e87d6c5417e0504d6ae7033ef6/Assets/Scripts/PiecesController.cs#L140-L147
        if is_key_pressed(self.settings.controls.rotate_clockwise) {
            self.rotate_tetrimino(true, true); // rotate clockwise
        } else if is_key_pressed(self.settings.controls.rotate_counterclockwise) {
            self.rotate_tetrimino(false, true); // rotate clockwise
        } else if is_key_pressed(self.settings.controls.hard_drop) {
            // the hard drop
            let mut y_offset = 0.0;
            for y in 0..HEIGHT as i32 {
                if !self.conflict(vec2(0.0, (y as f32) * BLOCK_SIZE)) {
                    y_offset = (y as f32) * BLOCK_SIZE as f32;
                }
            }
            for dot in self.active_piece.dots.iter_mut() {
                dot.y += y_offset;
            }
            self.set_active_tetrimino_position();
        }

        // handle line clears
        self.clear_lines();
    }

    fn new_tetrimino(&mut self) {
        let tetrimino_type =
            TETRIMINO_TYPES[macroquad::rand::gen_range::<usize>(0, TETRIMINO_TYPES.len())];
        self.active_piece = Piece {
            tetrimino: tetrimino_type,
            dots: tetrimino_type
                .get_structure()
                .iter()
                .map(|pos| {
                    vec2(
                        self.left_top_corner.x + (WIDTH / 2.0 * BLOCK_SIZE) + pos.x * BLOCK_SIZE,
                        self.left_top_corner.y + pos.y * BLOCK_SIZE,
                    )
                })
                .collect(),
            rotation_index: 0,
        };
    }

    pub fn draw_current_tetrimino(&self) {
        // draw current block
        self.active_piece.dots.iter().for_each(|position| {
            draw_rectangle(
                position.x,
                position.y,
                BLOCK_SIZE,
                BLOCK_SIZE,
                self.active_piece.tetrimino.get_color(),
            );
        });
    }

    pub fn draw_tetriminos(&self) {
        draw_rectangle(
            self.left_top_corner.x,
            self.left_top_corner.y,
            WIDTH * BLOCK_SIZE,
            HEIGHT * BLOCK_SIZE,
            GRAY,
        );

        // draw all the blocks that are already on the board
        for (y, row) in self.positions.iter().enumerate() {
            for (x, color) in row.iter().enumerate() {
                if let Some(color) = color {
                    draw_rectangle(
                        self.left_top_corner.x + x as f32 * BLOCK_SIZE,
                        self.left_top_corner.y + y as f32 * BLOCK_SIZE,
                        BLOCK_SIZE,
                        BLOCK_SIZE,
                        *color,
                    );
                }
            }
        }
    }

    pub fn get_relative_position_on_board(&self, absolute_position: Vec2) -> (usize, usize) {
        let relative_position = self
            .left_top_corner
            .abs()
            .sub(absolute_position.abs())
            .abs();
        let row = (relative_position.y / BLOCK_SIZE) as usize;
        let column = (relative_position.x / BLOCK_SIZE) as usize;
        (row, column)
    }

    // and x and y position are based off of the top left corner of the piece
    pub fn conflict(&self, relative_offset: Vec2) -> bool {
        self.active_piece
            .dots
            .iter()
            .map(|relative_position| relative_position.add(relative_offset))
            .any(|absolute| {
                let (row, column) = self.get_relative_position_on_board(absolute);
                (
                    absolute.x < self.left_top_corner.x // for the left side
               || absolute.x >=self.right_bottom_corner.x // for the right side
               || absolute.y >= self.right_bottom_corner.y
                    // for the floor (bottom)
                ) || self.positions[row][column].is_some()
            })
    }

    // https://github.com/JohnnyTurbo/LD43/blob/82de0ac5aa29f6e87d6c5417e0504d6ae7033ef6/Assets/Scripts/PieceController.cs#L249-L278
    pub fn rotate_tetrimino(&mut self, clockwise: bool, should_offset: bool) {
        let old_rotation_index = self.active_piece.rotation_index;
        self.active_piece.rotation_index += if clockwise { 1 } else { -1 };
        self.active_piece.rotation_index = self.mod_helper(self.active_piece.rotation_index, 4);
        let origin = self.active_piece.dots[0];
        for pos in &mut self.active_piece.dots {
            let relative_pos = pos.sub(origin);
            let rot_matrix = if clockwise {
                [vec2(0.0, -1.0), vec2(1.0, 0.0)]
            } else {
                [vec2(0.0, 1.0), vec2(-1.0, 0.0)]
            };
            *pos = vec2(
                (rot_matrix[0].x * relative_pos.x) + (rot_matrix[1].x * relative_pos.y) + origin.x,
                (rot_matrix[0].y * relative_pos.x) + (rot_matrix[1].y * relative_pos.y) + origin.y,
            );
        }
        if should_offset
            && !self.offset_tetrimino(old_rotation_index, self.active_piece.rotation_index)
        {
            self.rotate_tetrimino(!clockwise, false)
        }
    }

    fn mod_helper(&self, x: i8, m: i8) -> i8 {
        (x % m + m) % m
    }

    // https://github.com/JohnnyTurbo/LD43/blob/82de0ac5aa29f6e87d6c5417e0504d6ae7033ef6/Assets/Scripts/PieceController.cs#L297-L340
    fn offset_tetrimino(&mut self, old_rotation_index: i8, new_rotation_index: i8) -> bool {
        let offset_data = match self.active_piece.tetrimino {
            Tetrimino::O => crate::consts::Offset::O.to_vec(),
            Tetrimino::I => crate::consts::Offset::I.to_vec(),
            _ => crate::consts::Offset::JLSTZ.to_vec(),
        };

        let mut end_offset = vec2(0.0, 0.0);
        let mut move_possible = false;
        for test_index in 0..offset_data.len() {
            let offset_value1 = offset_data[test_index][old_rotation_index as usize];
            let offset_value2 = offset_data[test_index][new_rotation_index as usize];
            end_offset = offset_value2.sub(offset_value1);
            if !self.conflict(end_offset) {
                move_possible = true;
                break;
            }
        }

        if move_possible {
            for pos in self.active_piece.dots.iter_mut() {
                *pos = pos.add(end_offset);
            }
        }

        return move_possible;
    }

    pub fn set_active_tetrimino_position(&mut self) {
        for absolute_position in &self.active_piece.dots {
            let (row, column) = self.get_relative_position_on_board(*absolute_position);
            self.positions[row as usize][column as usize] =
                Some(self.active_piece.tetrimino.get_color());
        }
        self.new_tetrimino();
    }

    pub fn clear_lines(&mut self) {
        let mut index = HEIGHT - 1.0;
        while index >= 0.0 {
            let row = self.positions[index as usize];
            if row.iter().all(|x| x.is_some()) {
                self.clear_line(index as usize);
            } else {
                index -= 1.0;
            }
        }
    }

    fn clear_line(&mut self, row_index: usize) {
        for y in (1..row_index + 1).rev() {
            for x in 0..self.positions[y].len() {
                self.positions[y][x] = self.positions[y - 1][x];
            }
        }
        for x in 0..self.positions[0].len() {
            self.positions[0][x] = None;
        }
    }
}