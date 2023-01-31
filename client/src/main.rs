use macroquad::{
    main,
    prelude::{is_key_pressed, is_mouse_button_down, mouse_position, MouseButton, BLACK, WHITE},
    text::draw_text,
    time::get_fps,
    window::{clear_background, next_frame},
};

use tetris::{
    board::Board,
    consts::{vec2, GameState, Vec2, BLOCK_SIZE},
};

mod drawer;
mod game;
mod input;
mod settings;

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
                modify_board_bricks(left_top_corner, &mut game.board);
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

fn modify_board_bricks(left_top_corner: Vec2, board: &mut Board) {
    let (x, y) = mouse_position();
    let x = ((x - left_top_corner.x) / BLOCK_SIZE) as usize;
    let y = ((y - left_top_corner.y) / BLOCK_SIZE) as usize;
    let brick = vec![vec2(0.0, 0.0)];

    if is_mouse_button_down(MouseButton::Left)
        && !board.conflict(&brick, vec2(x as f32, y as f32), false)
    {
        board.add_brick(x, y, (105, 105, 105));
    } else if is_mouse_button_down(MouseButton::Right)
        && !board.conflict(&brick, vec2(x as f32, y as f32), false)
    {
        board.remove_brick(x, y);
    }
}
