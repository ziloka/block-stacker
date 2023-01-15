use std::ops::{Add, Sub};

use macroquad::{
    prelude::{draw_rectangle, vec2, Vec2, Color, GRAY},
    time::get_time,
};

use crate::consts::{Piece, Tetrimino, BLOCK_SIZE, HEIGHT, TETRIMINO_TYPES, WIDTH};

pub struct Board {
    pub left_top_corner: Vec2,
    pub right_bottom_corner: Vec2,
    pub time: f64,
    pub active_piece: Piece,
    positions: [[Color; WIDTH as usize]; HEIGHT as usize],
}

impl Board {
    pub fn new(left_top_corner: Vec2, right_bottom_corner: Vec2) -> Self {
      const CELL_INIT: Color = GRAY;
      const ROW_INIT: [Color; WIDTH as usize] = [CELL_INIT; WIDTH as usize];
        let tetrimino_type =
            TETRIMINO_TYPES[macroquad::rand::gen_range::<usize>(0, TETRIMINO_TYPES.len())];
        Self {
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
                    }).collect(),
                rotation_index: 0,
            },
            left_top_corner: left_top_corner,
            right_bottom_corner: right_bottom_corner,
            // https://stackoverflow.com/a/53930630
            positions: [ROW_INIT; HEIGHT as usize],
            time: get_time() * 1000.0,
        }
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

  pub fn draw_board_tetriminos(&self) {
      // draw all the blocks that are already on the board
      for (y, row) in self.positions.iter().enumerate() {
          for (x, color) in row.iter().enumerate() {
              if *color != GRAY {
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

    // and x and y position are based off of the top left corner of the piece
    pub fn conflict(&self, relative_offset: Vec2) -> bool {
        self.active_piece
            .dots
            .iter()
            .map(|pos| pos.add(relative_offset))
            .any(|e| {
                e.x < self.left_top_corner.x // for the left side
               || e.x >= self.right_bottom_corner.x // for the right side
               || e.y >= self.right_bottom_corner.y // for the floor (bottom)
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
            // move the piece
            for pos in self.active_piece.dots.iter_mut() {
                *pos = pos.add(end_offset);
            }
        }

        return move_possible;
    }

    pub fn set_active_tetrimino_position(&mut self) {
        for pos in &self.active_piece.dots {
            let x = (pos.x - self.left_top_corner.x) / BLOCK_SIZE;
            let y = (pos.y - self.left_top_corner.y) / BLOCK_SIZE;
            self.positions[y as usize][x as usize] =self.active_piece.tetrimino.get_color();
        }
        self.new_tetrimino();
    }
}
