use macroquad::{
    main,
    miniquad::date::now,
    prelude::{is_key_pressed, vec2, BLACK, WHITE},
    text::draw_text,
    time::get_fps,
    window::{clear_background, next_frame},
};

mod board;
mod consts;
mod drawer;
mod generator;
mod input;
mod settings;
mod utils;

use board::Board;
use consts::GameState;
use input::Input;

#[main("Tetris")]
async fn main() {
    let left_top_corner = vec2(200.0, 10.0);
    let mut board = Board::new(now() as u64, left_top_corner);
    let mut input = Input::default();

    loop {
        match board.game_state {
            GameState::OpenSettings => {
                board.settings.draw_menu();
            }
            GameState::Playing => {
                clear_background(BLACK);
                board.draw();
                input.handle(&mut board);
            }
            GameState::Paused => {
                todo!();
            }
            GameState::GameOver => {
                todo!();
            }
        }
        handle_keyboard_input(&mut board);
        draw_text(
            format!("fps: {}", get_fps()).as_str(),
            10.0,
            20.0,
            20.0,
            WHITE,
        );
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
