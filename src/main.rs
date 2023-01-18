use macroquad::{
    prelude::{vec2, BLACK, WHITE, is_key_pressed},
    text::draw_text,
    time::get_fps,
    window::{clear_background, next_frame},
    miniquad::date::now,
    main
};

mod consts;
use consts::{GameState, BLOCK_SIZE, HEIGHT, WIDTH};

mod board;
use board::Board;

mod settings;
mod generator;

#[main("Tetris")]
async fn main() {
    let mut board = Board::new(
      now() as u64,
        vec2(200.0, 20.0),
        vec2(200.0 + WIDTH * BLOCK_SIZE, 20.0 + HEIGHT * BLOCK_SIZE),
    );

    loop {
        match board.game_state {
            GameState::OpenSettings => {
                board.settings.draw_menu();
            }
            GameState::Playing => {
                clear_background(BLACK);
                draw_text(
                    format!("fps: {}", get_fps()).as_str(),
                    10.0,
                    20.0,
                    20.0,
                    WHITE,
                );
                board.draw_tetriminos();
                board.draw_current_tetrimino();
                board.handle_movement();
            }
            GameState::Paused => {
                todo!();
            }
            GameState::GameOver => {
                todo!();
            }
        }
        handle_keyboard_input(&mut board);

        next_frame().await;
    }
}

fn handle_keyboard_input(board: &mut Board) {
    if is_key_pressed(macroquad::input::KeyCode::Escape) {
        match board.game_state {
            GameState::Playing => {
                board.game_state = GameState::OpenSettings;
            }
            GameState::OpenSettings => {
                board.game_state = GameState::Playing;
            }
            _ => panic!("Not implemented yet"),
        }
    }
}
