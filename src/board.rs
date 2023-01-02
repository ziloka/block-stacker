use macroquad::{
    prelude::{draw_rectangle, vec2, Vec2},
    time::get_time,
};

use crate::consts::{Piece, Tetrimino, BLOCK_SIZE, HEIGHT, TETRIMINO_TYPES, WIDTH};

pub struct Board {
    pub left_top_corner: Vec2,
    pub right_bottom_corner: Vec2,
    pub time: f64,
    pub active_piece: Piece,
    positions: [[u8; WIDTH as usize]; HEIGHT as usize],
}

impl Board {
    pub fn new(left_top_corner: Vec2, right_bottom_corner: Vec2) -> Self {
        let tetrimino_type =
            TETRIMINO_TYPES[macroquad::rand::gen_range::<usize>(0, TETRIMINO_TYPES.len())];
        Self {
            active_piece: Piece {
                tetrimino: tetrimino_type,
                dots: tetrimino_type
                    .get_structure()
                    .iter()
                    .map(|pos| {
                        vec2(
                            left_top_corner.x + (WIDTH / 2.0) + pos.x * BLOCK_SIZE,
                            left_top_corner.y + pos.y * BLOCK_SIZE,
                        )
                    })
                    .collect::<Vec<_>>(),
                rotation_index: 0,
            },
            left_top_corner: left_top_corner,
            right_bottom_corner: right_bottom_corner,
            // https://stackoverflow.com/a/53930630
            positions: [[0; WIDTH as usize]; HEIGHT as usize],
            time: get_time() * 1000.0,
        }
    }

    pub fn draw_tetrimino(&self) {
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

    // https://github.com/JohnnyTurbo/LD43/blob/82de0ac5aa29f6e87d6c5417e0504d6ae7033ef6/Assets/Scripts/PieceController.cs#L249-L278
    pub fn rotate_tetrimino(&mut self, clockwise: bool, should_offset: bool) {
        let old_rotation_index = self.active_piece.rotation_index;
        self.active_piece.rotation_index += if clockwise { 1 } else { -1 };
        self.active_piece.rotation_index = self.mod_helper(self.active_piece.rotation_index, 4);
        let origin = self.active_piece.dots[0];
        for pos in &mut self.active_piece.dots {
            let relative_pos = vec2(pos.x - origin.x, pos.y - origin.y);
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
            end_offset = vec2(
                offset_value2.x - offset_value1.x,
                offset_value2.y - offset_value1.y,
            );
            if !self.conflict(end_offset.x, end_offset.y) {
                move_possible = true;
                break;
            }
        }

        if move_possible {
            // move the piece
            for pos in self.active_piece.dots.iter_mut() {
                pos.x += end_offset.x;
                pos.y += end_offset.y;
            }
        }

        return move_possible;
    }

    // and x and y position are based off of the top left corner of the piece
    pub fn conflict(&self, x_offset: f32, y_offset: f32) -> bool {
        self.active_piece
            .dots
            .iter()
            .map(|pos| vec2(pos.x + x_offset, pos.y + y_offset))
            .any(|e| {
                e.x <= self.left_top_corner.x // for the left side
             || e.x >= self.right_bottom_corner.x // for the right side
             || e.y >= self.right_bottom_corner.y // for the floor (bottom)
            })
    }
}
