use macroquad::{
    prelude::{vec2, BLACK, WHITE},
    rand,
    text::draw_text,
    time::get_fps,
    window::{clear_background, next_frame},
};

mod consts;
use consts::{BLOCK_SIZE, HEIGHT, WIDTH};

mod board;
use board::Board;

#[macroquad::main("Tetris")]
async fn main() {
    rand::srand(macroquad::miniquad::date::now() as u64);
    let mut board = Board::new(
        vec2(200.0, 20.0),
        vec2(200.0 + WIDTH * BLOCK_SIZE, 20.0 + HEIGHT * BLOCK_SIZE),
    );

    loop {
        clear_background(BLACK);
        draw_text(
            format!("fps: {}", get_fps()).as_str(),
            10.0,
            20.0,
            20.0,
            WHITE,
        );
        board.draw_board_tetriminos();
        board.draw_current_tetrimino();
        board.handle_movement();
        next_frame().await;
    }
}
