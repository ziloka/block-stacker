use std::cell::Cell;

use macroquad::{
    main,
    prelude::{is_key_pressed, is_mouse_button_down, mouse_position, KeyCode, MouseButton, BLACK},
    window::{clear_background, next_frame, screen_height, screen_width},
};

use tetris::{
    core::logic::{
        board::Board,
        consts::{vec2, State, Vec2, CUSTOM_GARBAGE},
    },
    drawer::Drawer,
    game::{Bot, Game, Human, Room},
};

#[main("Tetris")]
async fn main() {
    let bottom_left_corner = Cell::new(vec2(0.0, 0.0));
    let block_size = Cell::new(30.0);
    let debug = Cell::new(false);

    let mut open_settings = false;
    let mut game = Game::default();

    // open game menu
    let mut room = Room::new(0, "Room 1".to_string(), 4, false, false);
    let mut human = Box::new(Human::default());

    room.add_player(human);
    room.add_player(Box::new(Bot::default()));

    // game.add_player(&room);
    // game.add_player(&room);

    room.start();
}
