use std::ops::{Add, Sub};

use super::{
    action::Action,
    consts::{vec2, Piece, State, Tetromino, Vec2, HEIGHT, TETROMINO_TYPES, WIDTH},
    drawer::Drawer,
    generator::Generator,
};

pub struct Board<'a> {
    pub game_state: State,
    pub active_piece: Piece,
    pub hold_piece: Option<Piece>,
    generator: Generator,
    pub drawer: &'a dyn Drawer,
    preview_pieces: [Tetromino; 7],
    positions: [[Option<(u8, u8, u8)>; WIDTH as usize]; HEIGHT as usize],
}

impl<'a> Board<'a> {
    pub fn new(drawer: &'a dyn Drawer, seed: usize) -> Self {
        const CELL_INIT: Option<(u8, u8, u8)> = None;
        const ROW_INIT: [Option<(u8, u8, u8)>; WIDTH as usize] = [CELL_INIT; WIDTH as usize];
        let positions = [ROW_INIT; HEIGHT as usize];
        // this first place gets replaced so it doesn't matter what it is
        let active_piece = Piece {
            tetromino: TETROMINO_TYPES[0],
            dots: TETROMINO_TYPES[0]
                .get_structure()
                .iter()
                .map(|pos| vec2(pos.x + WIDTH / 2.0, pos.y + 1.0))
                .collect(),
            rotation_index: 0,
            previous_rotation_index: None,
        };
        let mut generator = Generator::new(seed);
        let tetrominos = generator.get_new_sequence_of_tetrominos();
        let mut board = Self {
            game_state: State::Playing,
            active_piece,
            hold_piece: None,
            generator,
            drawer,
            preview_pieces: tetrominos,
            // https://stackoverflow.com/a/53930630
            positions,
        };
        board.set_next_tetromino_as_active_piece();
        board
    }

    pub fn draw(&self) {
        self.drawer.draw_tetrominos(&self.positions);
        self.drawer.draw_current_tetromino(&self.active_piece);
        self.drawer.draw_preview_pieces(&self.preview_pieces);
        self.drawer.draw_hold_piece(&self.hold_piece);
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
        let piece = Piece {
            tetromino: self.preview_pieces[0],
            dots: self.preview_pieces[0]
                .get_structure()
                .iter()
                .map(|pos| vec2(pos.x + WIDTH / 2.0, pos.y + 1.0))
                .collect(),
            rotation_index: 0,
            previous_rotation_index: None,
        };
        if self.conflict(&piece.dots, vec2(0.0, 0.0), false) {
            self.game_state = State::GameOver;
        } else {
            self.active_piece = piece;
            self.add_tetromino_preview_piece();
        }
    }

    fn add_tetromino_preview_piece(&mut self) {
        for i in 1..self.preview_pieces.len() {
            self.preview_pieces[i - 1] = self.preview_pieces[i];
        }
        self.preview_pieces[self.preview_pieces.len() - 1] = self.generator.next().unwrap();
    }

    // and x and y position are based off of the top left corner of the piece
    pub fn conflict(
        &self,
        positions: &[Vec2],
        relative_offset: Vec2,
        allow_overlapping_blocks: bool,
    ) -> bool {
        positions
            .iter()
            .map(|relative_position| relative_position.add(relative_offset))
            .any(|relative| {
                let Vec2 { x: column, y: row } = relative;
                (
                    relative.x < 0.0 // for the left side
               || relative.x >= WIDTH // for the right side
               || relative.y >= HEIGHT
                    // for the floor (bottom)
                ) || allow_overlapping_blocks
                    && self.positions[row as usize][column as usize].is_some()
            })
    }

    // https://harddrop.com/wiki/SRS
    // https://github.com/JohnnyTurbo/LD43/blob/82de0ac5aa29f6e87d6c5417e0504d6ae7033ef6/Assets/Scripts/PieceController.cs#L249-L278
    pub fn rotate_tetromino(&mut self, clockwise: bool, should_offset: bool) {
        let old_rotation_index = self.active_piece.rotation_index;
        self.active_piece.previous_rotation_index = Some(old_rotation_index);
        self.active_piece.rotation_index += if clockwise { -1 } else { 1 };
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
            )
            .add(origin);
        }

        if should_offset
            && !self.offset_tetromino(old_rotation_index, self.active_piece.rotation_index)
        {
            self.rotate_tetromino(!clockwise, false);
        }
    }

    // determine the rotation index of the tetromino
    // returning 0 - O means it is in its spawn position
    // returning 1 - R means it is in a clockwise rotation ("right") from spawn
    // returning 2 - L means it is in a counter-clockwise rotation ("left") from spawn
    // returning 3 - 2 means it is in a 180 degree rotation from spawn
    fn mod_helper(&self, x: i8, m: i8) -> i8 {
        (x % m + m) % m
    }

    // https://github.com/JohnnyTurbo/LD43/blob/82de0ac5aa29f6e87d6c5417e0504d6ae7033ef6/Assets/Scripts/PieceController.cs#L297-L340
    // https://github.com/fiorescarlatto/four-tris/blob/master/Tetris.au3#L3395-L3449
    fn offset_tetromino(&mut self, old_rotation_index: i8, new_rotation_index: i8) -> bool {
        let offset_data = self.active_piece.tetromino.get_offset_data();
        let mut move_possible = false;

        for offset_element in offset_data {
            let offset_value1 = offset_element[old_rotation_index as usize];
            let offset_value2 = offset_element[new_rotation_index as usize];
            let end_offset = vec2(
                offset_value1.x - offset_value2.x,
                (offset_value1.y - offset_value2.y) * -1.0,
            );

            if !self.conflict(&self.active_piece.dots, end_offset, true) {
                // https://github.com/JohnnyTurbo/LD43/blob/82de0ac5aa29f6e87d6c5417e0504d6ae7033ef6/Assets/Scripts/PieceController.cs#L226-L247
                for dot in self.active_piece.dots.iter_mut() {
                    *dot = dot.add(end_offset);
                }
                move_possible = true;
                break;
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
        self.clear_lines();
        self.set_next_tetromino_as_active_piece();
    }

    fn determine_action_performed(&self, lines_cleared: u8) -> Option<Action> {
        if matches!(self.active_piece.tetromino, Tetromino::T) {
            let Vec2 { x, y } = self.active_piece.dots[0];
            let brick = vec![vec2(0.0, 0.0)];
            let top_left_filled = self.conflict(&brick, vec2(x - 1.0, y - 1.0), false);
            let top_right_filled = self.conflict(&brick, vec2(x + 1.0, y - 1.0), false);
            let bottom_left_filled = self.conflict(&brick, vec2(x - 1.0, y + 1.0), false);
            let bottom_right_filled = self.conflict(&brick, vec2(x + 1.0, y + 1.0), false);

            if (bottom_left_filled && bottom_right_filled) && (top_left_filled && !top_right_filled || !top_left_filled && top_right_filled)
            {
                // check if i was suppose to upgrade TSpinMini to regular Tspin
                // T has to use the fifth, or the final, kick, a Mini T-Spin gets bumped to a T-Spin even if the corners were filled on the back rather than front
                if lines_cleared == 1 {
                    return Some(Action::TSpinMiniSingle);
                } else if lines_cleared == 2 {
                    return Some(Action::TSpinMiniDouble);
                }
                return Some(Action::TSpinMiniNoLines);
            }
            if lines_cleared == 1 {
                return Some(Action::TSpinSingle);
            } else if lines_cleared == 2 {
                return Some(Action::TSpinDouble);
            } else if lines_cleared == 3 {
                return Some(Action::TSpinTriple);
            }
            return Some(Action::TSpinNoLines);
        } else if lines_cleared == 1 {
            return Some(Action::Single);
        } else if lines_cleared == 2 {
            return Some(Action::Double);
        } else if lines_cleared == 3 {
            return Some(Action::Triple);
        } else if lines_cleared == 4 {
            return Some(Action::Tetris);
        }
        None
    }

    fn clear_lines(&mut self) {
        let mut lines_cleared = 0;
        let mut index = HEIGHT - 1.0;
        while index >= 0.0 {
            let row = self.positions[index as usize];
            if row.iter().all(|x| x.is_some()) {
                self.clear_line(index as usize);
                lines_cleared += 1;
            } else {
                index -= 1.0;
            }
        }
        if let Some(action) = self.determine_action_performed(lines_cleared) {
            println!("action: {}", action.to_string());
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

    pub fn add_brick(&mut self, x: usize, y: usize, color: (u8, u8, u8)) {
        self.positions[y][x] = Some(color);
    }

    pub fn remove_brick(&mut self, x: usize, y: usize) {
        self.positions[y][x] = None;
    }
}
