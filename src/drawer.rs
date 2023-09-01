use std::cell::Cell;

use macroquad::{
    prelude::{draw_rectangle, Color, BLACK, GREEN, ORANGE, RED, VIOLET, WHITE},
    shapes::{draw_circle_lines, draw_rectangle_lines},
    text::draw_text,
};

use crate::tetris::{
    board::Board,
    consts::{vec2, Piece, Tetromino, Vec2, GRAY, HEIGHT, WIDTH},
};

pub struct Drawer<'a> {
    pub bottom_left_corner: &'a Cell<Vec2>,
    pub block_size: &'a Cell<f32>,
    pub debug: &'a Cell<bool>,
}

impl<'a> crate::tetris::drawer::Drawer for Drawer<'a> {
    fn draw_current_tetromino(&self, active_piece: &Piece) {
        let block_size = self.block_size.get();
        let bottom_left_corner = self.bottom_left_corner.get();
        let debug = self.debug.get();
        let (r, g, b) = active_piece.tetromino.get_color();
        let origin = active_piece.dots[0];

        if debug {
            // this is just for debugging purposes (ensure pieces are in the true SRS rotations)
            // https://harddrop.com/wiki/File:SRS-true-rotations.png
            let (min, max) = match active_piece.tetromino {
                Tetromino::I => (-2, 2),
                _ => (-1, 1),
            };
            // draw black rectangle thing behind tetromino
            for i in min..=max {
                for j in min..=max {
                    let x = bottom_left_corner.x + (origin.x + i as f32) * block_size;
                    let y = bottom_left_corner.y - (origin.y + j as f32) * block_size - block_size;
                    draw_rectangle(x, y, block_size, block_size, BLACK);
                }
            }
        }

        // draw current block
        active_piece.dots.iter().for_each(|position| {
            let x = bottom_left_corner.x + position.x * block_size;
            let y = bottom_left_corner.y - position.y * block_size - block_size;
            draw_rectangle(x, y, block_size, block_size, Color::from_rgba(r, g, b, 255));

            draw_rectangle_lines(x, y, block_size, block_size, 2.0, BLACK);
        });

        if debug {
            // draw circle on origin on active_piece (continued debug purposes)
            draw_circle_lines(
                bottom_left_corner.x + origin.x * block_size + block_size / 2.0,
                bottom_left_corner.y - origin.y * block_size + block_size / 2.0 - block_size,
                block_size / 2.0,
                2.0,
                WHITE,
            );

            if matches!(active_piece.tetromino, Tetromino::T) {
                let fl = vec2(origin.x - 1.0, origin.y + 1.0) * block_size;
                let fr = vec2(origin.x + 1.0, origin.y + 1.0) * block_size;
                let bl = vec2(origin.x + 1.0, origin.y - 1.0) * block_size;
                let br = vec2(origin.x - 1.0, origin.y - 1.0) * block_size;

                let (
                    front_left_corner_filled,
                    front_right_corner_filled,
                    bottom_left_corner_filled,
                    bottom_right_corner_filled,
                ) = match active_piece.rotation_index {
                    0 => (fl, fr, bl, br),
                    1 => (fr, br, fl, bl),
                    2 => (br, bl, fr, fl),
                    _ => {
                        // make the assumption its rotation index 3
                        assert!(active_piece.rotation_index == 3);
                        (bl, fl, br, fr)
                    }
                };

                draw_rectangle_lines(
                    bottom_left_corner.x + front_left_corner_filled.x,
                    bottom_left_corner.y - front_left_corner_filled.y - block_size,
                    block_size,
                    block_size,
                    5.0,
                    RED,
                );
                draw_rectangle_lines(
                    bottom_left_corner.x + front_right_corner_filled.x,
                    bottom_left_corner.y - front_right_corner_filled.y - block_size,
                    block_size,
                    block_size,
                    5.0,
                    ORANGE,
                );
                draw_rectangle_lines(
                    bottom_left_corner.x + bottom_left_corner_filled.x,
                    bottom_left_corner.y - bottom_left_corner_filled.y - block_size,
                    block_size,
                    block_size,
                    5.0,
                    GREEN,
                );
                draw_rectangle_lines(
                    bottom_left_corner.x + bottom_right_corner_filled.x,
                    bottom_left_corner.y - bottom_right_corner_filled.y - block_size,
                    block_size,
                    block_size,
                    5.0,
                    VIOLET,
                );
            }
        }
    }

    // draw ghost piece, ghost piece appears where the harddrop would be
    fn draw_ghost_piece(&self, board: &Board, active_piece: &Piece) {
        let block_size = self.block_size.get();
        let bottom_left_corner = self.bottom_left_corner.get();

        let mut y_offset = 0.0;
        for y in 1..HEIGHT as i32 {
            let y = y as f32 * -1.0;
            if board.conflict(&active_piece.dots, vec2(0.0, y), true) {
                y_offset = y + 1.0;
                break;
            }
        }

        active_piece.dots.iter().for_each(|position| {
            let x = bottom_left_corner.x + position.x * block_size;
            let y = bottom_left_corner.y - (position.y + y_offset) * block_size - block_size;
            draw_rectangle(x, y, block_size, block_size, WHITE);
            draw_rectangle_lines(x, y, block_size, block_size, 2.0, BLACK);
        });
    }

    fn draw_tetrominos(
        &self,
        positions: &[[Option<(u8, u8, u8)>; WIDTH as usize]; HEIGHT as usize],
    ) {
        let block_size = self.block_size.get();
        let bottom_left_corner = self.bottom_left_corner.get();
        let debug = self.debug.get();

        draw_rectangle(
            bottom_left_corner.x,
            bottom_left_corner.y - HEIGHT * block_size,
            WIDTH * block_size,
            HEIGHT * block_size,
            Color::from_rgba(GRAY.0, GRAY.1, GRAY.2, 255),
        );

        // draw all the blocks that are already on the board
        for (y, row) in positions.iter().enumerate() {
            for (x, color) in row.iter().enumerate() {
                let x = bottom_left_corner.x + x as f32 * block_size;
                let y = bottom_left_corner.y - (y as f32 + 1.0) * block_size;
                if let Some(color) = color {
                    let (r, g, b) = *color;
                    draw_rectangle(x, y, block_size, block_size, Color::from_rgba(r, g, b, 255));
                }
                draw_rectangle_lines(x, y, block_size, block_size, 2.0, BLACK);
            }
        }

        if debug {
            // draw the indexs on the side of the board
            for i in 0..(HEIGHT as u32) {
                draw_text(
                    &i.to_string(),
                    bottom_left_corner.x - block_size,
                    (bottom_left_corner.y - i as f32 * block_size) - block_size / 2.0,
                    20.0,
                    WHITE,
                );
            }

            for i in 0..(WIDTH as u32) {
                draw_text(
                    &i.to_string(),
                    bottom_left_corner.x + i as f32 * block_size,
                    bottom_left_corner.y + block_size / 2.0,
                    20.0,
                    WHITE,
                );
            }
        }
    }

    fn draw_preview_pieces(&self, preview_pieces: &[Tetromino; 7]) {
        let block_size = self.block_size.get();
        let bottom_left_corner = self.bottom_left_corner.get();

        preview_pieces[0..5]
            .iter()
            .enumerate()
            .for_each(|(i, tetromino)| {
                tetromino.get_structure().iter().for_each(|position| {
                    let x = bottom_left_corner.x + (WIDTH + 2.0 + position.x) * block_size;
                    let y = (bottom_left_corner.y - HEIGHT * block_size)
                        + (i as f32 * 3.0 + position.y) * block_size;
                    let (r, g, b) = tetromino.get_color();
                    draw_rectangle(x, y, block_size, block_size, Color::from_rgba(r, g, b, 255));

                    draw_rectangle_lines(x, y, block_size, block_size, 2.0, BLACK);
                });
            });
    }

    fn draw_hold_piece(&self, hold_piece: &Option<Piece>) {
        let block_size = self.block_size.get();
        let bottom_left_corner = self.bottom_left_corner.get();
        let debug = self.debug.get();

        if let Some(piece) = hold_piece {
            let dots = piece.tetromino.get_structure();
            let origin = dots[0];
            dots.iter().for_each(|position| {
                let x = bottom_left_corner.x - (origin.x - position.x * block_size) + block_size * -3.0;
                let y = (bottom_left_corner.y - origin.y - HEIGHT * block_size)
                    + position.y * block_size
                    + block_size;
                let (r, g, b) = piece.tetromino.get_color();
                draw_rectangle(x, y, block_size, block_size, Color::from_rgba(r, g, b, 255));

                draw_rectangle_lines(x, y, block_size, block_size, 2.0, BLACK);
            });

            if debug {
                // draw origin dot on hold piece location
                draw_circle_lines(
                    bottom_left_corner.x - origin.x * block_size + block_size * -3.0 + (block_size / 2.0),
                    (bottom_left_corner.y - origin.y - HEIGHT * block_size) - (block_size / 2.0 - block_size * 2.0),
                    block_size / 2.0,
                    2.0,
                    WHITE
                );
            }

        }
    }

    fn draw_action_text(&self, text: &str) {
        let block_size = self.block_size.get();
        let bottom_left_corner = self.bottom_left_corner.get();

        draw_text(
            text,
            bottom_left_corner.x - 5.0 * block_size,
            bottom_left_corner.y - (HEIGHT / 2.0) * block_size,
            20.0,
            WHITE,
        );
    }
}
