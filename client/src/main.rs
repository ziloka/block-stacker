use macroquad::{
    main,
    prelude::{is_key_pressed, BLACK, WHITE},
    text::draw_text,
    time::get_fps,
    window::{clear_background, next_frame},
};

use tetris::{board::Board, consts::{vec2, GameState}};

mod drawer;
mod input;
mod settings;
mod game;

use game::Game;

#[main("Tetris")]
async fn main() {
    let left_top_corner = vec2(200.0, 10.0);
    let mut game = Game::new(left_top_corner);

    loop {
        match game.board.game_state {
            GameState::OpenSettings => {
                game.input.settings.draw_menu();
            }
            GameState::Playing => {
                clear_background(BLACK);
                game.input.handle(&mut game.board);
                game.board.draw(&game.drawer);
            }
            GameState::Paused => {
                todo!();
            }
            GameState::GameOver => {
                todo!();
            }
        }
        handle_keyboard_input(&mut game.board);
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
