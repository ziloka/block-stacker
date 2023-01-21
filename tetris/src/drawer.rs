use crate::consts::{WIDTH, HEIGHT, Piece, Tetrimino};

pub trait Drawer {
    fn draw_current_tetrimino(&self, active_piece: &Piece);
    fn draw_tetriminos(&self, positions: &[[Option<(u8, u8, u8)>; WIDTH as usize]; HEIGHT as usize]);
    fn draw_preview_pieces(&self, preview_pieces: &[Tetrimino; 7]);
    fn draw_hold_piece(&self, hold_piece: &Option<Piece>);
}