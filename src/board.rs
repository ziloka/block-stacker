use std::ops::{Add, Sub};

use macroquad::{
    prelude::{vec2, Color, Vec2},
    rand::srand,
};

use crate::{
    consts::{GameState, Piece, Tetrimino, BLOCK_SIZE, HEIGHT, TETRIMINO_TYPES, WIDTH},
    drawer::Drawer,
    generator::Generator,
    input::Input,
    settings::Settings,
};

pub struct Board {
    pub game_state: GameState,
    pub settings: Settings,
    pub active_piece: Piece,
    pub hold_piece: Option<Piece>,
    pub movement: Input,
    left_top_corner: Vec2,
    right_bottom_corner: Vec2,
    drawer: Drawer,
    generator: Generator,
    preview_pieces: [Tetrimino; 7],
    positions: [[Option<Color>; WIDTH as usize]; HEIGHT as usize],
}

const CELL_INIT: Option<Color> = None;
const ROW_INIT: [Option<Color>; WIDTH as usize] = [CELL_INIT; WIDTH as usize];

impl Board {
    pub fn new(seed: u64, left_top_corner: Vec2, right_bottom_corner: Vec2) -> Self {
        srand(seed);
        let positions = [ROW_INIT; HEIGHT as usize];
        // this first place gets replaced so it doesn't matter what it is
        let active_piece = Piece {
            tetrimino: TETRIMINO_TYPES[0],
            dots: TETRIMINO_TYPES[0]
                .get_structure()
                .iter()
                .map(|pos| {
                    vec2(
                        left_top_corner.x + (WIDTH / 2.0 * BLOCK_SIZE) + pos.x * BLOCK_SIZE,
                        left_top_corner.y + pos.y * BLOCK_SIZE,
                    )
                })
                .collect(),
            rotation_index: 0,
        };
        let mut generator = Generator::new(seed);
        let tetriminos = generator.get_new_sequence_of_tetriminos();
        let mut board = Self {
            game_state: GameState::Playing,
            settings: Settings::default(),
            active_piece,
            hold_piece: None,
            movement: Input::default(),
            left_top_corner,
            right_bottom_corner,
            drawer: Drawer { left_top_corner },
            generator,
            preview_pieces: tetriminos,
            // https://stackoverflow.com/a/53930630
            positions,
        };
        board.set_next_tetrimino_as_active_piece();
        board
    }

    pub fn draw_tetriminos(&self) {
        self.drawer.draw_tetriminos(&self.positions);
    }

    pub fn draw_current_tetrimino(&self) {
        self.drawer.draw_current_tetrimino(&self.active_piece);
    }

    pub fn hold_tetrimino(&mut self) {
        if self.hold_piece.is_none() {
            self.hold_piece = Some(self.active_piece.clone());
            self.set_next_tetrimino_as_active_piece();
        } else {
            let mut hold_piece = self.hold_piece.clone().unwrap();
            hold_piece.dots = hold_piece
                .tetrimino
                .get_structure()
                .iter()
                .map(|pos| {
                    vec2(
                        self.left_top_corner.x + (WIDTH / 2.0 * BLOCK_SIZE) + pos.x * BLOCK_SIZE,
                        self.left_top_corner.y + pos.y * BLOCK_SIZE,
                    )
                })
                .collect();
            hold_piece.rotation_index = 0;
            self.hold_piece = Some(self.active_piece.clone());
            self.active_piece = hold_piece;
        }
    }

    fn set_next_tetrimino_as_active_piece(&mut self) {
        self.active_piece = Piece {
            tetrimino: self.preview_pieces[0],
            dots: self.preview_pieces[0]
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
        self.add_tetrimino_preview_piece();
    }

    fn add_tetrimino_preview_piece(&mut self) {
        for i in 1..self.preview_pieces.len() {
            self.preview_pieces[i - 1] = self.preview_pieces[i];
        }
        self.preview_pieces[self.preview_pieces.len() - 1] = self.generator.next();
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
        let offset_data = self.active_piece.tetrimino.get_offset_data();

        let mut end_offset = vec2(0.0, 0.0);
        let mut move_possible = false;

        for offset_element in offset_data {
            let offset_value1 = offset_element[old_rotation_index as usize];
            let offset_value2 = offset_element[new_rotation_index as usize];
            end_offset = offset_value2.sub(offset_value1);
            if !self.conflict(end_offset) {
                move_possible = true;
                break;
            }
        }

        // TODO: implement wallkicks / figure out why this doesn't work
        if move_possible {
            // https://github.com/JohnnyTurbo/LD43/blob/82de0ac5aa29f6e87d6c5417e0504d6ae7033ef6/Assets/Scripts/PieceController.cs#L226-L247
            if !self.conflict(end_offset) && end_offset == vec2(0.0, -1.0) {
                for dot in self.active_piece.dots.iter_mut() {
                    *dot = dot.add(end_offset * BLOCK_SIZE);
                }
                self.set_active_tetrimino_position();
            }
        }

        move_possible
    }

    pub fn set_active_tetrimino_position(&mut self) {
        for absolute_position in &self.active_piece.dots {
            let (row, column) = self.get_relative_position_on_board(*absolute_position);
            self.positions[row][column] = Some(self.active_piece.tetrimino.get_color());
        }
        self.set_next_tetrimino_as_active_piece();
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
