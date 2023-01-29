use std::ops::{Add, Sub};

use crate::{
    consts::{vec2, GameState, Piece, Tetromino, Vec2, HEIGHT, TETROMINO_TYPES, WIDTH},
    drawer::Drawer,
    generator::Generator,
};

pub struct Board {
    pub game_state: GameState,
    pub active_piece: Piece,
    pub hold_piece: Option<Piece>,
    generator: Generator,
    preview_pieces: [Tetromino; 7],
    positions: [[Option<(u8, u8, u8)>; WIDTH as usize]; HEIGHT as usize],
}

impl Board {
    pub fn new(seed: usize) -> Self {
        const CELL_INIT: Option<(u8, u8, u8)> = None;
        const ROW_INIT: [Option<(u8, u8, u8)>; WIDTH as usize] = [CELL_INIT; WIDTH as usize];
        let positions = [ROW_INIT; HEIGHT as usize];
        // this first place gets replaced so it doesn't matter what it is
        let active_piece = Piece {
            tetromino: TETROMINO_TYPES[0],
            dots: TETROMINO_TYPES[0]
                .get_structure()
                .iter()
                .map(|pos| vec2(pos.x + WIDTH / 2.0, pos.y))
                .collect(),
            rotation_index: 0,
        };
        let mut generator = Generator::new(seed);
        let tetrominos = generator.get_new_sequence_of_tetrominos();
        let mut board = Self {
            game_state: GameState::Playing,
            active_piece,
            hold_piece: None,
            generator,
            preview_pieces: tetrominos,
            // https://stackoverflow.com/a/53930630
            positions,
        };
        board.set_next_tetromino_as_active_piece();
        board
    }

    pub fn draw(&self, drawer: &impl Drawer) {
        drawer.draw_tetrominos(&self.positions);
        drawer.draw_current_tetromino(&self.active_piece);
        drawer.draw_preview_pieces(&self.preview_pieces);
        drawer.draw_hold_piece(&self.hold_piece);
    }

    pub fn hold_tetromino(&mut self) {
        if self.hold_piece.is_none() {
            self.hold_piece = Some(self.active_piece.clone());
            self.set_next_tetromino_as_active_piece();
        } else {
            let mut hold_piece = self.hold_piece.clone().unwrap();
            hold_piece.dots = hold_piece
                .tetromino
                .get_structure()
                .iter()
                .map(|pos| vec2(pos.x + WIDTH / 2.0, pos.y))
                .collect();
            hold_piece.rotation_index = 0;
            self.hold_piece = Some(self.active_piece.clone());
            self.active_piece = hold_piece;
        }
    }

    fn set_next_tetromino_as_active_piece(&mut self) {
        self.active_piece = Piece {
            tetromino: self.preview_pieces[0],
            dots: self.preview_pieces[0]
                .get_structure()
                .iter()
                .map(|pos| vec2(pos.x + WIDTH / 2.0, pos.y))
                .collect(),
            rotation_index: 0,
        };
        self.add_tetromino_preview_piece();
    }

    fn add_tetromino_preview_piece(&mut self) {
        for i in 1..self.preview_pieces.len() {
            self.preview_pieces[i - 1] = self.preview_pieces[i];
        }
        self.preview_pieces[self.preview_pieces.len() - 1] = self.generator.next().unwrap();
    }

    // and x and y position are based off of the top left corner of the piece
    pub fn conflict(&self, relative_offset: Vec2) -> bool {
        self.active_piece
            .dots
            .iter()
            .map(|relative_position| relative_position.add(relative_offset))
            .any(|relative| {
                let Vec2 { x: column, y: row } = relative;
                (
                    relative.x < 0.0 // for the left side
               || relative.x >= WIDTH // for the right side
               || relative.y >= HEIGHT
                    // for the floor (bottom)
                ) || self.positions[row as usize][column as usize].is_some()
            })
    }

    // https://github.com/JohnnyTurbo/LD43/blob/82de0ac5aa29f6e87d6c5417e0504d6ae7033ef6/Assets/Scripts/PieceController.cs#L249-L278
    pub fn rotate_tetromino(&mut self, clockwise: bool, should_offset: bool) {
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
                (rot_matrix[0].x * relative_pos.x) + (rot_matrix[1].x * relative_pos.y),
                (rot_matrix[0].y * relative_pos.x) + (rot_matrix[1].y * relative_pos.y),
            ).add(origin);
        }

        if should_offset
            && !self.offset_tetromino(old_rotation_index, self.active_piece.rotation_index)
        {
            self.rotate_tetromino(!clockwise, false);
        }
    }

    fn mod_helper(&self, x: i8, m: i8) -> i8 {
        (x % m + m) % m
    }

    // https://github.com/JohnnyTurbo/LD43/blob/82de0ac5aa29f6e87d6c5417e0504d6ae7033ef6/Assets/Scripts/PieceController.cs#L297-L340
    fn offset_tetromino(&mut self, old_rotation_index: i8, new_rotation_index: i8) -> bool {
        let offset_data = self.active_piece.tetromino.get_offset_data();
        let mut end_offset = vec2(0.0, 0.0);
        let mut move_possible = false;
        
        for offset_element in offset_data {
            let offset_value1 = offset_element[old_rotation_index as usize];
            let offset_value2 = offset_element[new_rotation_index as usize];
            end_offset = offset_value1.sub(offset_value2);
            if !self.conflict(end_offset) {
                move_possible = true;
                break;
            }
        }

        // TODO: implement wallkicks / figure out why this doesn't work
        if move_possible {
            // println!("offset: {:?} condition: {:?}", end_offset, self.conflict(end_offset));
            // https://github.com/JohnnyTurbo/LD43/blob/82de0ac5aa29f6e87d6c5417e0504d6ae7033ef6/Assets/Scripts/PieceController.cs#L226-L247
            if !self.conflict(end_offset) {
                for dot in self.active_piece.dots.iter_mut() {
                    *dot = dot.add(end_offset);
                }
            }
        }

        move_possible
    }

    pub fn set_active_tetromino_position(&mut self) {
        for relative_position in &self.active_piece.dots {
            let Vec2 { x: column, y: row } = *relative_position;
            self.positions[row as usize][column as usize] =
                Some(self.active_piece.tetromino.get_color());
        }
        self.set_next_tetromino_as_active_piece();
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
