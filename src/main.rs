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
        Position { x: 0.0, y: 0.0 },
        Position {
            x: 0.0 + WIDTH * BLOCK_SIZE,
            y: 0.0 + HEIGHT * BLOCK_SIZE,
        },
    );
    loop {
        // draw background & board
        clear_background(BLACK);
        // draw rectangle from top left corner
        draw_rectangle(0.0, 0.0, WIDTH * BLOCK_SIZE, HEIGHT * BLOCK_SIZE, GRAY);

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
        if is_key_down(KeyCode::Left)
            && !board.conflict(
                board.active_piece.position.x - BLOCK_SIZE,
                board.active_piece.position.y,
            )
        {
            board.active_piece.position.x -= BLOCK_SIZE;
        } else if is_key_down(KeyCode::Right)
            && !board.conflict(
                board.active_piece.position.x + BLOCK_SIZE,
                board.active_piece.position.y,
            )
        {
            board.active_piece.position.x += BLOCK_SIZE;
        } else if is_key_down(KeyCode::Down)
            && !board.conflict(
                board.active_piece.position.x,
                board.active_piece.position.y + BLOCK_SIZE,
            )
        {
            board.active_piece.position.y += BLOCK_SIZE;
        }
        board.time = current_time;
    }
}
