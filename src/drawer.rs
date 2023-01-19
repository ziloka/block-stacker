use macroquad::prelude::{draw_rectangle, Color, Vec2, GRAY};

use crate::consts::{Piece, BLOCK_SIZE, HEIGHT, WIDTH};

pub struct Drawer {
    pub left_top_corner: Vec2,
}

impl Drawer {
    pub fn draw_current_tetrimino(&self, active_piece: &Piece) {
        // draw current block
        active_piece.dots.iter().for_each(|position| {
            draw_rectangle(
                position.x,
                position.y,
                BLOCK_SIZE,
                BLOCK_SIZE,
                active_piece.tetrimino.get_color(),
            );
        });
    }

    pub fn draw_tetriminos(&self, positions: &[[Option<Color>; WIDTH as usize]; HEIGHT as usize]) {
        draw_rectangle(
            self.left_top_corner.x,
            self.left_top_corner.y,
            WIDTH * BLOCK_SIZE,
            HEIGHT * BLOCK_SIZE,
            GRAY,
        );

        // draw all the blocks that are already on the board
        for (y, row) in positions.iter().enumerate() {
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
}
