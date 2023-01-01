use consts::{ARR, BLOCK_SIZE, DAS, HEIGHT, WIDTH};
// https://github.com/not-fl3/macroquad/blob/master/examples/input_keys.rs
use macroquad::prelude::*;
use std::time::SystemTime;

mod consts;
use consts::Position;

mod board;
use board::Board;

#[macroquad::main("Tetris")]
async fn main() {
    let mut board = Board::new(
        Position { x: 200.0, y: 20.0 },
        Position {
            x: 200.0 + WIDTH * BLOCK_SIZE,
            y: 20.0 + HEIGHT * BLOCK_SIZE,
        },
    );

    loop {
        // draw background & board
        clear_background(BLACK);
        // draw rectangle from top left corner
        draw_rectangle(
            board.left_top_corner.x,
            board.left_top_corner.y,
            WIDTH * BLOCK_SIZE,
            HEIGHT * BLOCK_SIZE,
            GRAY,
        );

        // draw fps
        draw_text(
            format!("fps: {}", get_fps()).as_str(),
            10.0,
            20.0,
            20.0,
            WHITE,
        );

        // draw active piece
        board.draw_tetrimino();

        handle_movement(&mut board);
        next_frame().await;
    }
}

fn handle_movement(board: &mut Board) {
    let current_time = SystemTime::now();
    // https://www.reddit.com/r/Tetris/comments/frbii6/comment/fphx9ml/?utm_source=share&utm_medium=web2x&context=3
    if (is_key_down(KeyCode::Left) || is_key_down(KeyCode::Right) || is_key_down(KeyCode::Down))
        && (current_time.duration_since(board.time).unwrap().as_millis() > DAS as u128
            || ((!is_key_pressed(KeyCode::Left)
                || !is_key_pressed(KeyCode::Right)
                || !is_key_pressed(KeyCode::Down))
                && current_time.duration_since(board.time).unwrap().as_millis() > ARR as u128))
    {
        if is_key_down(KeyCode::Left) && !board.conflict(-BLOCK_SIZE, 0.0) {
            for dot in board.active_piece.dots.iter_mut() {
                dot.x -= BLOCK_SIZE;
            }
        } else if is_key_down(KeyCode::Right) && !board.conflict(BLOCK_SIZE, 0.0) {
            for dot in board.active_piece.dots.iter_mut() {
                dot.x += BLOCK_SIZE;
            }
        } else if is_key_down(KeyCode::Down) && !board.conflict(0.0, BLOCK_SIZE) {
            for dot in board.active_piece.dots.iter_mut() {
                dot.y += BLOCK_SIZE;
            }
        }
        board.time = current_time;
    }

    if is_key_pressed(KeyCode::Up) {
        board.rotate_tetrimino(true, true) // rotate clockwise
    } else if is_key_pressed(KeyCode::Z) {
    }
}
