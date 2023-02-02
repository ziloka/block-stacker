use std::cell::Cell;

use macroquad::{
    prelude::{draw_rectangle, Color, BLACK, GRAY, WHITE},
    shapes::{draw_circle_lines, draw_rectangle_lines},
};

use tetris::consts::{Piece, Tetromino, Vec2, BLOCK_SIZE, HEIGHT, WIDTH};

pub struct Drawer<'a> {
    pub left_top_corner: Vec2,
    pub debug: &'a Cell<bool>,
}

impl<'a> tetris::drawer::Drawer for Drawer<'a> {
    fn draw_current_tetromino(&self, active_piece: &Piece) {
        let (r, g, b) = active_piece.tetromino.get_color();
        let origin = active_piece.dots[0];

        if self.debug.get() {
            // this is just for debugging purposes (ensure pieces are in the true SRS rotations)
            // https://harddrop.com/wiki/File:SRS-true-rotations.png
            let (min, max) = match active_piece.tetromino {
                Tetromino::I => (-2, 2),
                _ => (-1, 1),
            };
            for i in min..=max {
                for j in min..=max {
                    let x = self.left_top_corner.x + (origin.x + i as f32) * BLOCK_SIZE;
                    let y = self.left_top_corner.y + (origin.y + j as f32) * BLOCK_SIZE;
                    draw_rectangle(x, y, BLOCK_SIZE, BLOCK_SIZE, BLACK);
                }
            }
        }

        // draw current block
        active_piece.dots.iter().for_each(|position| {
            let x = self.left_top_corner.x + position.x * BLOCK_SIZE;
            let y = self.left_top_corner.y + position.y * BLOCK_SIZE;
            draw_rectangle(x, y, BLOCK_SIZE, BLOCK_SIZE, Color::from_rgba(r, g, b, 255));

            draw_rectangle_lines(x, y, BLOCK_SIZE, BLOCK_SIZE, 2.0, BLACK);
        });

        if self.debug.get() {
            // draw origin circle on active_piece (continued debug purposes)
            draw_circle_lines(
                self.left_top_corner.x + origin.x * BLOCK_SIZE + BLOCK_SIZE / 2.0,
                self.left_top_corner.y + origin.y * BLOCK_SIZE + BLOCK_SIZE / 2.0,
                BLOCK_SIZE / 2.0,
                2.0,
                WHITE,
            )
        }
    }

    fn draw_tetrominos(
        &self,
        positions: &[[Option<(u8, u8, u8)>; WIDTH as usize]; HEIGHT as usize],
    ) {
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
                let x = self.left_top_corner.x + x as f32 * BLOCK_SIZE;
                let y = self.left_top_corner.y + y as f32 * BLOCK_SIZE;
                if let Some(color) = color {
                    let (r, g, b) = *color;
                    draw_rectangle(x, y, BLOCK_SIZE, BLOCK_SIZE, Color::from_rgba(r, g, b, 255));
                }
                draw_rectangle_lines(x, y, BLOCK_SIZE, BLOCK_SIZE, 2.0, BLACK);
            }
        }
    }

    fn draw_preview_pieces(&self, preview_pieces: &[Tetromino; 7]) {
        preview_pieces
            .iter()
            .enumerate()
            .for_each(|(i, tetromino)| {
                tetromino.get_structure().iter().for_each(|position| {
                    let x = self.left_top_corner.x + (WIDTH + 2.0 + position.x) * BLOCK_SIZE;
                    let y = self.left_top_corner.y + (i as f32 * 3.0 + position.y) * BLOCK_SIZE;
                    let (r, g, b) = tetromino.get_color();
                    draw_rectangle(x, y, BLOCK_SIZE, BLOCK_SIZE, Color::from_rgba(r, g, b, 255));

                    draw_rectangle_lines(x, y, BLOCK_SIZE, BLOCK_SIZE, 2.0, BLACK);
                });
            });
    }

    fn draw_hold_piece(&self, hold_piece: &Option<Piece>) {
        if let Some(piece) = hold_piece {
            piece.tetromino.get_structure().iter().for_each(|position| {
                let x = self.left_top_corner.x - position.x * BLOCK_SIZE + BLOCK_SIZE * -4.0;
                let y = self.left_top_corner.y - position.y * BLOCK_SIZE + BLOCK_SIZE;
                let (r, g, b) = piece.tetromino.get_color();
                draw_rectangle(x, y, BLOCK_SIZE, BLOCK_SIZE, Color::from_rgba(r, g, b, 255));

                draw_rectangle_lines(x, y, BLOCK_SIZE, BLOCK_SIZE, 2.0, BLACK);
            });
        }
    }
}
