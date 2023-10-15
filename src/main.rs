use std::cell::Cell;

use macroquad::{
    main,
    prelude::{
        is_key_pressed, is_mouse_button_down, mouse_position, KeyCode, MouseButton, BLACK, WHITE, get_time
    },
    window::{clear_background, next_frame, screen_height, screen_width},
};

mod drawer;
mod game;
mod history;
mod input;
mod settings;
mod tetris;

use crate::{
    drawer::Drawer,
    game::Game,
    tetris::{
        board::Board,
        consts::{vec2, State, Vec2, CUSTOM_GARBAGE, HEIGHT, WIDTH},
    },
};

#[main("Tetris")]
async fn main() {
    let bottom_left_corner = Cell::new(vec2(0.0, 0.0));
    let block_size = Cell::new(30.0);
    let debug = Cell::new(false);
    let drawer = Drawer {
        start: get_time(),
        bottom_left_corner: &bottom_left_corner,
        block_size: &block_size,
        debug: &debug,
    };
    let mut open_settings = false;
    let mut game = Game::new(&drawer);

    loop {
        let block_size_temp =
            (screen_height() / (HEIGHT * 1.25)).min(screen_width() / (WIDTH * 1.25));
        block_size.set(block_size_temp);
        bottom_left_corner.set(vec2(
            block_size_temp * 6.0,
            block_size_temp * (HEIGHT + 2.0),
        ));

        if open_settings {
            game.input.settings.draw_menu();
        } else {
            match game.board.game_state {
                State::Playing => {
                    clear_background(BLACK);
                    modify_board_bricks(&bottom_left_corner, &mut game.board, &block_size);
                    game.input.handle(&mut game.board);
                    game.board.draw();
                }
                State::GameOver => {
                    todo!();
                }
            }
        }
        handle_keyboard_input(&mut game.board, &debug, &mut open_settings);
        next_frame().await;
    }
}

fn handle_keyboard_input(board: &mut Box<Board>, debug: &Cell<bool>, open_settings: &mut bool) {
    if is_key_pressed(KeyCode::Escape) {
        match board.game_state {
            State::Playing => {
                *open_settings = !*open_settings;
            }
            _ => unimplemented!(),
        }
    } else if is_key_pressed(KeyCode::F3) {
        debug.set(!debug.get());
    }
}

fn modify_board_bricks(bottom_left_corner: &Cell<Vec2>, board: &mut Box<Board>, block_size: &Cell<f32>) {
    let block_size = block_size.get();
    let bottom_left_corner = bottom_left_corner.get();
    let (x, y) = mouse_position();
    let x = ((x - bottom_left_corner.x) / block_size).floor() as usize;
    let y = ((bottom_left_corner.y - y) / block_size).floor() as usize;
    let brick = vec![vec2(0.0, 0.0)];

    if is_mouse_button_down(MouseButton::Left)
        && !board.conflict(&brick, vec2(x as f32, y as f32), false)
    {
        board.add_brick(x, y, CUSTOM_GARBAGE);
    } else if is_mouse_button_down(MouseButton::Right)
        && !board.conflict(&brick, vec2(x as f32, y as f32), false)
    {
        board.remove_brick(x, y);
    }
}
