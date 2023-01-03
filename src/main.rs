use macroquad::{
    prelude::{is_key_down, is_key_pressed, vec2, KeyCode, BLACK, GRAY, WHITE},
    shapes::draw_rectangle,
    text::draw_text,
    time::get_fps,
    window::{clear_background, next_frame}, rand,
};

mod consts;
use consts::{ARR, BLOCK_SIZE, DAS, HEIGHT, WIDTH};

mod board;
use board::Board;

#[macroquad::main("Tetris")]
async fn main() {
    // make randomly generated numbers pseudo random (based off of the current time which always changes)
    // https://github.com/not-fl3/macroquad/issues/369
    // https://github.com/not-fl3/macroquad/issues/519
    rand::srand(macroquad::miniquad::date::now() as u64);

    let mut board = Board::new(
        vec2(200.0, 20.0),
        vec2(200.0 + WIDTH * BLOCK_SIZE, 20.0 + HEIGHT * BLOCK_SIZE),
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

        board.draw_board_tetriminos();

        // draw active piece
        board.draw_tetrimino();

        handle_movement(&mut board);
        next_frame().await;
    }
}

fn handle_movement(board: &mut Board) {
    let current_time = macroquad::time::get_time() * 1000.0; // time in miliseconds since the start of the program
    // https://www.reddit.com/r/Tetris/comments/frbii6/comment/fphx9ml/?utm_source=share&utm_medium=web2x&context=3
    if (is_key_down(KeyCode::Left) || is_key_down(KeyCode::Right) || is_key_down(KeyCode::Down))
        && (current_time - board.time > DAS as f64
            || ((!is_key_pressed(KeyCode::Left)
                || !is_key_pressed(KeyCode::Right)
                || !is_key_pressed(KeyCode::Down))
                && current_time - board.time > ARR as f64))
    {
        if is_key_down(KeyCode::Left) && !board.conflict(vec2(-BLOCK_SIZE, 0.0)) {
            for dot in board.active_piece.dots.iter_mut() {
                dot.x -= BLOCK_SIZE;
            }
        } else if is_key_down(KeyCode::Right) && !board.conflict(vec2(BLOCK_SIZE, 0.0)) {
            for dot in board.active_piece.dots.iter_mut() {
                dot.x += BLOCK_SIZE;
            }
        } else if is_key_down(KeyCode::Down) && !board.conflict(vec2(0.0, BLOCK_SIZE)) {
            for dot in board.active_piece.dots.iter_mut() {
                dot.y += BLOCK_SIZE;
            }
        }
        board.time = current_time;
    }

    // https://github.com/JohnnyTurbo/LD43/blob/82de0ac5aa29f6e87d6c5417e0504d6ae7033ef6/Assets/Scripts/PiecesController.cs#L140-L147
    if is_key_pressed(KeyCode::Up) {
        board.rotate_tetrimino(true, true) // rotate clockwise
    } else if is_key_pressed(KeyCode::Z) {
        board.rotate_tetrimino(false, true) // rotate clockwise
    } else if is_key_pressed(KeyCode::Space) { // the hard drop
        let mut y_offset = 0;
        for y in (0..HEIGHT as i32).rev() {
          if !board.conflict(vec2(0.0, (y * BLOCK_SIZE as i32) as f32)) {
            y_offset = y * BLOCK_SIZE as i32;
            break;
          }
        }
        for dot in board.active_piece.dots.iter_mut() {
            dot.y += y_offset as f32;
        }
        board.set_active_tetrimino_position();
    }
}
