use std::{
    fmt::Debug,
    fmt::{Display, Formatter},
    ops::{Add, Sub},
};

use super::{
    action::Action,
    consts::{vec2, Piece, State, Tetromino, Vec2},
    generator::Generator,
};

#[derive(Clone)]
pub struct Board {
    pub game_state: State,
    pub active_piece: Piece,
    pub hold_piece: Option<Piece>,
    pub score: u64,
    generator: Generator,
    pub preview_pieces: [Tetromino; 7],
    // (0, 0) represents the bottom left corner, (WIDTH, HEIGHT) represents the top right corner
    pub positions: Vec<Vec<Option<(u8, u8, u8)>>>,
    pub last_action: Option<Action>,
}

impl Display for Board {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut board_string = String::new();
        for row in self.positions.iter() {
            for element in row.iter().rev() {
                if let Some(_) = element {
                    board_string.insert_str(0, "x");
                } else {
                    board_string.insert_str(0, " ");
                }
            }
            board_string.insert_str(0, "\n");
        }
        write!(f, "{}", board_string)
    }
}

impl Debug for Board {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut board_string = String::new();
        for row in self.positions.iter() {
            for element in row.iter().rev() {
                if let Some(_) = element {
                    board_string.insert_str(0, "x");
                } else {
                    board_string.insert_str(0, " ");
                }
            }
            board_string.insert_str(0, "\n");
        }
        write!(f, "{}", board_string)
    }
}

impl Board {
    pub fn import(positions: Vec<Vec<Option<(u8, u8, u8)>>>, seed: usize) -> Self {
        let active_piece: Piece = Default::default();
        let mut generator = Generator::new(seed);
        let tetrominos = generator.get_new_sequence_of_tetrominos();
        let mut board = Self {
            game_state: State::Playing,
            active_piece,
            hold_piece: None,
            score: 0,
            generator,
            preview_pieces: tetrominos,
            // https://stackoverflow.com/a/53930630
            positions: positions,
            last_action: None,
        };
        board.set_next_tetromino_as_active_piece();
        board
    }

    pub fn new(seed: usize) -> Self {
        // this first place gets replaced so it doesn't matter what it is
        let active_piece: Piece = Default::default();
        let mut generator = Generator::new(seed);
        let tetrominos = generator.get_new_sequence_of_tetrominos();
        let mut board = Self {
            game_state: State::Playing,
            active_piece,
            hold_piece: None,
            score: 0,
            generator,
            preview_pieces: tetrominos,
            // https://stackoverflow.com/a/53930630
            positions: vec![vec![None; 10]; 20],
            last_action: None,
        };
        board.set_next_tetromino_as_active_piece();
        board
    }

    pub fn restart(&mut self, seed: usize) {
        let mut generator = Generator::new(seed);
        let tetrominos = generator.get_new_sequence_of_tetrominos();
        self.game_state = State::Playing;
        self.hold_piece = None;
        self.generator = generator;
        self.preview_pieces = tetrominos;
        self.positions = vec![vec![None; self.positions[0].len()]; self.positions.len()];
        self.set_next_tetromino_as_active_piece();
    }

    fn get_spawn_dots(tetromino: &Tetromino, board_height: f32, board_width: f32) -> Vec<Vec2> {
        tetromino
            .get_structure()
            .iter()
            .map(|pos| vec2(pos.x + board_width as f32 / 2.0, board_height - 2.0 - pos.y))
            .collect()
    }

    pub fn hold_tetromino(&mut self) {
        match &mut self.hold_piece {
            Some(hold_piece) => {
                hold_piece.rotation_index = 0;
                hold_piece.dots = Self::get_spawn_dots(
                    &hold_piece.tetromino,
                    self.positions.len() as f32,
                    self.positions[0].len() as f32,
                );
                std::mem::swap(hold_piece, &mut self.active_piece);
            }
            None => {
                self.hold_piece = Some(self.active_piece.clone());
                self.set_next_tetromino_as_active_piece();
            }
        }
    }

    fn set_next_tetromino_as_active_piece(&mut self) {
        let piece = Piece {
            tetromino: self.preview_pieces[0],
            dots: Self::get_spawn_dots(
                &self.preview_pieces[0],
                self.positions.len() as f32,
                self.positions[0].len() as f32,
            ),
            rotation_index: 0,
            previous_rotation_index: None,
            previous_offset_kick: None,
        };
        if self.conflict(&piece.dots, vec2(0.0, 0.0), true) {
            // self.game_state = State::GameOver;
            println!("UNCOMMENT ME RIGHT NOW");
        } else {
            self.active_piece = piece;
            self.add_tetromino_preview_piece();
        }
    }

    fn add_tetromino_preview_piece(&mut self) {
        self.preview_pieces.rotate_left(1);
        self.preview_pieces[self.preview_pieces.len() - 1] = self.generator.next().unwrap();
    }

    // and x and y position are based off of the bottom left corner of the piece
    pub fn conflict(
        &self,
        dots: &[Vec2],
        relative_offset: Vec2,
        dont_allow_overlapping_blocks: bool,
    ) -> bool {
        dots.iter()
            .map(|relative_position| relative_position.add(relative_offset))
            .any(|relative| {
                let Vec2 { x: column, y: row } = relative;
                (
                    relative.x < 0.0 // for the left side
                 || relative.x >= self.positions[0].len() as f32 // for the right side
                 || relative.y < 0.0
                    // for the floor (bottom)
                ) || dont_allow_overlapping_blocks
                    && self.positions[row as usize][column as usize].is_some()
            })
    }

    pub fn move_left(&mut self) {
        for dot in self.active_piece.dots.iter_mut() {
            dot.x -= 1.0;
        }
    }

    pub fn move_right(&mut self) {
        for dot in self.active_piece.dots.iter_mut() {
            dot.x += 1.0;
        }
    }

    pub fn soft_drop(&mut self) {
        for dot in self.active_piece.dots.iter_mut() {
            dot.y -= 1.0;
        }
    }

    pub fn hard_drop(&mut self) {
        // the hard drop
        let mut y_offset = 0.0;
        for y in 1..self.positions.len() as i32 {
            let y = y as f32 * -1.0;
            if self.conflict(&self.active_piece.dots, vec2(0.0, y), true) {
                y_offset = y + 1.0;
                break;
            }
        }
        for dot in self.active_piece.dots.iter_mut() {
            dot.y += y_offset;
        }
        self.set_active_tetromino_position();
    }

    pub fn rotate_tetromino_90(&mut self, clockwise: bool, should_offset: bool) -> bool {
        let old_rotation_index = self.active_piece.rotation_index;
        self.active_piece.previous_rotation_index = Some(old_rotation_index);
        self.active_piece.rotation_index += if clockwise { 1 } else { -1 };
        self.active_piece.rotation_index = self.mod_helper(self.active_piece.rotation_index, 4);
        let origin = self.active_piece.dots[0];
        let rot_matrix = if clockwise {
            [vec2(0.0, -1.0), vec2(1.0, 0.0)]
        } else {
            [vec2(0.0, 1.0), vec2(-1.0, 0.0)]
        };
        for pos in &mut self.active_piece.dots {
            let relative_pos = pos.sub(origin);
            *pos = vec2(
                (rot_matrix[0].x * relative_pos.x) + (rot_matrix[1].x * relative_pos.y),
                (rot_matrix[0].y * relative_pos.x) + (rot_matrix[1].y * relative_pos.y),
            )
            .add(origin);
        }

        if should_offset
            && !self.wallkick_tetromino(
                self.active_piece
                    .tetromino
                    .get_offset_data()
                    .get(Tetromino::find_offset_row_90(
                        old_rotation_index,
                        self.active_piece.rotation_index,
                    ))
                    .unwrap(),
            )
        {
            self.rotate_tetromino_90(!clockwise, false);
            return true;
        }
        return false;
    }

    pub fn rotate_tetromino_180(&mut self, should_offset: bool) -> bool {
        let old_rotation_index = self.active_piece.rotation_index;
        self.active_piece.previous_rotation_index = Some(old_rotation_index);
        self.active_piece.rotation_index += 2;
        self.active_piece.rotation_index = self.mod_helper(self.active_piece.rotation_index, 4);
        let origin = self.active_piece.dots[0];
        let rot_matrix = [vec2(0.0, -1.0), vec2(1.0, 0.0)];
        for _ in 0..2 {
            for pos in &mut self.active_piece.dots {
                let relative_pos = pos.sub(origin);
                *pos = vec2(
                    (rot_matrix[0].x * relative_pos.x) + (rot_matrix[1].x * relative_pos.y),
                    (rot_matrix[0].y * relative_pos.x) + (rot_matrix[1].y * relative_pos.y),
                )
                .add(origin);
            }
        }
        if should_offset
            && !self.wallkick_tetromino(
                self.active_piece
                    .tetromino
                    .get_offset_data()
                    .get(Tetromino::find_offset_row_180(
                        old_rotation_index,
                        self.active_piece.rotation_index,
                    ))
                    .unwrap(),
            )
        {
            self.rotate_tetromino_180(false);
            return true;
        }
        return false;

        // self.rotate_tetromino(clockwise, true)
    }

    // determine the rotation index of the tetromino
    // returning 0 - O means it is in its spawn position
    // returning 1 - R means it is in a clockwise rotation ("right") from spawn
    // returning 3 - L means it is in a counter-clockwise rotation ("left") from spawn
    // returning 2 - 2 means it is in a 180 degree rotation from spawn
    fn mod_helper(&self, x: i8, m: i8) -> i8 {
        (x % m + m) % m
    }

    fn wallkick_tetromino(&mut self, wallkick_data: &Vec<Vec2>) -> bool {
        let mut move_possible = false;
        for offset_element in wallkick_data {
            if !self.conflict(&self.active_piece.dots, *offset_element, true) {
                // self.active_piece.previous_offset_kick = Some(i);
                for dot in self.active_piece.dots.iter_mut() {
                    *dot = dot.add(*offset_element);
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

    fn determine_action_performed(&self, lines_cleared: usize) -> Option<Action> {
        let brick: Vec<Vec2> = vec![vec2(0.0, 0.0)];
        if matches!(self.active_piece.tetromino, Tetromino::T)
            && self.active_piece.previous_rotation_index.is_some()
        {
            let Vec2 { x, y } = self.active_piece.dots[0];
            let fl = self.conflict(&brick, vec2(x - 1.0, y + 1.0), true);
            let fr = self.conflict(&brick, vec2(x + 1.0, y + 1.0), true);
            let br = self.conflict(&brick, vec2(x + 1.0, y - 1.0), true);
            let bl = self.conflict(&brick, vec2(x - 1.0, y - 1.0), true);

            let (
                front_left_corner_filled,
                front_right_corner_filled,
                bottom_left_corner_filled,
                bottom_right_corner_filled,
            ) = match self.active_piece.rotation_index {
                0 => (fl, fr, bl, br),
                1 => (br, fl, fr, bl),
                2 => (bl, br, fl, fr),
                _ => {
                    // make the assumption its rotation index 3
                    assert!(self.active_piece.rotation_index == 3);
                    (fr, bl, br, fl)
                }
            };

            if (front_left_corner_filled && front_right_corner_filled)
                && (bottom_left_corner_filled ^ bottom_right_corner_filled)
            {
                if lines_cleared == 0 {
                    return Some(Action::TSpinNoLines);
                } else if lines_cleared == 1 {
                    return Some(Action::TSpinSingle);
                } else if lines_cleared == 2 {
                    return Some(Action::TSpinDouble);
                } else if lines_cleared == 3 {
                    return Some(Action::TSpinTriple);
                }
            } else if (front_left_corner_filled ^ front_right_corner_filled)
                && (bottom_left_corner_filled && bottom_right_corner_filled)
            {
                // check whether or not upgrade TSpinMini to regular Tspin
                // T has to use the fifth, or the final, kick, a Mini T-Spin gets bumped to a T-Spin even if the corners were filled on the back rather than front
                if self
                    .active_piece
                    .previous_offset_kick
                    .is_some_and(|num| num == 4)
                {
                    if lines_cleared == 0 {
                        return Some(Action::TSpinNoLines);
                    } else if lines_cleared == 1 {
                        return Some(Action::TSpinSingle);
                    } else if lines_cleared == 2 {
                        return Some(Action::TSpinDouble);
                    } else if lines_cleared == 3 {
                        return Some(Action::TSpinTriple);
                    }
                }

                if lines_cleared == 0 {
                    return Some(Action::TSpinMiniNoLines);
                } else if lines_cleared == 1 {
                    return Some(Action::TSpinMiniSingle);
                } else if lines_cleared == 2 {
                    return Some(Action::TSpinMiniDouble);
                }
            }
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
        let line_clear_to_be_cleared = self
            .positions
            .iter()
            .filter(|row| row.iter().all(|x| x.is_some()))
            .collect::<Vec<_>>()
            .len();
        if let Some(action) = self.determine_action_performed(line_clear_to_be_cleared) {
            self.last_action = Some(action);
        }

        let mut index = 0.0;
        while index <= self.positions.len() as f32 - 1.0 {
            let row = &self.positions[index as usize];
            if row.iter().all(|x| x.is_some()) {
                self.clear_line(index as usize);
            } else {
                index += 1.0;
            }
        }
    }

    fn clear_line(&mut self, row_index: usize) {
        for y in row_index..self.positions.len() as usize - 1 {
            self.positions[y] = self.positions[y + 1].clone();
        }
    }

    pub fn add_brick(&mut self, x: usize, y: usize, color: (u8, u8, u8)) {
        self.positions[y][x] = Some(color);
    }

    pub fn remove_brick(&mut self, x: usize, y: usize) {
        self.positions[y][x] = None;
    }
}
