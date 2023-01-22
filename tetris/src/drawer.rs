use crate::consts::{WIDTH, HEIGHT, Piece, Tetromino};

pub trait Drawer {
    fn draw_current_tetromino(&self, active_piece: &Piece);
    fn draw_tetrominos(&self, positions: &[[Option<(u8, u8, u8)>; WIDTH as usize]; HEIGHT as usize]);
    fn draw_preview_pieces(&self, preview_pieces: &[Tetromino; 7]);
    fn draw_hold_piece(&self, hold_piece: &Option<Piece>);
}