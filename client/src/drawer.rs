use macroquad::prelude::{draw_rectangle, Color, GRAY};

use tetris::consts::{Piece, Tetrimino, Vec2, BLOCK_SIZE, HEIGHT, WIDTH};

pub struct Drawer {
    pub left_top_corner: Vec2,
}

impl tetris::drawer::Drawer for Drawer {
    fn draw_current_tetrimino(&self, active_piece: &Piece) {
        let (r, g, b) = active_piece.tetrimino.get_color();
        // draw current block
        active_piece.dots.iter().for_each(|position| {
            draw_rectangle(
                self.left_top_corner.x + position.x * BLOCK_SIZE,
                self.left_top_corner.y + position.y * BLOCK_SIZE,
                BLOCK_SIZE,
                BLOCK_SIZE,
                Color::new(r as f32, g as f32, b as f32, 0.0),
            );
        });
    }

    fn draw_tetriminos(&self, positions: &[[Option<(u8, u8, u8)>; WIDTH as usize]; HEIGHT as usize]) {
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
                    let (r, g, b) = *color;
                    draw_rectangle(
                        self.left_top_corner.x + x as f32 * BLOCK_SIZE,
                        self.left_top_corner.y + y as f32 * BLOCK_SIZE,
                        BLOCK_SIZE,
                        BLOCK_SIZE,
                        Color::new(r as f32, g as f32, b as f32, 0.0),
                    );
                }
            }
        }
    }

    fn draw_preview_pieces(&self, preview_pieces: &[Tetrimino; 7]) {
        preview_pieces
            .iter()
            .enumerate()
            .for_each(|(i, tetrimino)| {
                tetrimino.get_structure().iter().for_each(|position| {
                    let (r, g, b) = tetrimino.get_color();
                    draw_rectangle(
                        self.left_top_corner.x + (WIDTH + 2.0 + position.x) * BLOCK_SIZE,
                        self.left_top_corner.y + (i as f32 * 3.0 + position.y) * BLOCK_SIZE,
                        BLOCK_SIZE,
                        BLOCK_SIZE,
                        Color::new(r as f32, g as f32, b as f32, 0.0),
                    );
                });
            });
    }

    fn draw_hold_piece(&self, hold_piece: &Option<Piece>) {
        if let Some(piece) = hold_piece {
            piece.tetrimino.get_structure().iter().for_each(|position| {
                let (r, g, b) = piece.tetrimino.get_color();
                draw_rectangle(
                    self.left_top_corner.x - position.x * BLOCK_SIZE + BLOCK_SIZE * -4.0,
                    self.left_top_corner.y - position.y * BLOCK_SIZE + BLOCK_SIZE,
                    BLOCK_SIZE,
                    BLOCK_SIZE,
                    Color::new(r as f32, g as f32, b as f32, 0.0),
                );
            });
        }
    }
}