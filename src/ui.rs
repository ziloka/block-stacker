use std::{
    cell::Cell,
    fmt::{self, Display, Formatter},
};

use macroquad::{
    color::BLACK,
    input::{is_key_pressed, is_mouse_button_down, mouse_position},
    miniquad::{EventHandler, KeyMods, MouseButton},
    prelude::{
        get_last_key_pressed,
        utils::{register_input_subscriber, repeat_all_miniquad_input},
        KeyCode,
    },
    ui::{hash, root_ui, widgets::Window},
    window::{clear_background, next_frame, screen_height, screen_width},
};

use crate::{
    core::logic::{
        board::Board,
        consts::{State, Vec2, CUSTOM_GARBAGE},
    },
    drawer::Drawer,
    game::{Game, Human, Player},
};

pub struct Settings {
    pub handles: Handles,
    pub controls: Controls,
    focused_on: Option<FocusedOn>,
    subscriber_id: usize,
}

// https://tetris.wiki/DAS
// https://www.reddit.com/r/Tetris/comments/frbii6/comment/fphx9ml?context=3
// https://www.reddit.com/r/Tetris/comments/13uqby/comment/c77ev43/?context=3
pub struct Handles {
    pub das: f32, // Delayed Auto Shift (the time between the initial keypress and the start of its automatic repeat movement) in frames per movement
    pub arr: f32, // Auto Repeat Rate (the speed at which tetroiminoes move when holding down movement keys) in frames per movement
}

pub struct Controls {
    pub left: KeyCode,
    pub right: KeyCode,
    pub soft_drop: KeyCode,
    pub hard_drop: KeyCode,
    pub rotate_clockwise: KeyCode,
    pub rotate_counterclockwise: KeyCode,
    pub rotate_180: KeyCode,
    pub hold: KeyCode,
    pub restart: KeyCode,
    pub undo: KeyCode,
    pub redo: KeyCode,
}

enum FocusedOn {
    Left,
    Right,
    SoftDrop,
    HardDrop,
    RotateClockwise,
    RotateCounterclockwise,
    Rotate180,
    Hold,
    Restart,
    Undo,
    Redo,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            handles: Handles {
                das: 133.0,
                arr: 10.0,
            },
            controls: Controls {
                left: KeyCode::Left,
                right: KeyCode::Right,
                soft_drop: KeyCode::Down,
                hard_drop: KeyCode::Space,
                rotate_clockwise: KeyCode::Up,
                rotate_counterclockwise: KeyCode::X,
                rotate_180: KeyCode::A,
                hold: KeyCode::C,
                restart: KeyCode::R,
                undo: KeyCode::F4,
                redo: KeyCode::F5,
            },
            focused_on: None,
            subscriber_id: register_input_subscriber(),
        }
    }
}

impl EventHandler for Settings {
    fn update(&mut self) {}
    fn draw(&mut self) {}

    fn char_event(&mut self, _character: char, _keymods: KeyMods, _repeat: bool) {}
}

impl Settings {
    pub fn draw_menu(&mut self) {
        Window::new(
            hash!(),
            macroquad::prelude::vec2(0., 0.),
            macroquad::prelude::vec2(300., 800.),
        )
        .label("Settings")
        .titlebar(true)
        .ui(&mut root_ui(), |ui| {
            ui.tree_node(hash!(), "handles", |ui| {
                ui.slider(
                    hash!(),
                    "DAS (frames per movement)",
                    0.0..500.0,
                    &mut self.handles.das,
                );
                ui.slider(
                    hash!(),
                    "ARR (frames per movement)",
                    0.0..500.,
                    &mut self.handles.arr,
                );
            });
            ui.separator();
            ui.tree_node(hash!(), "controls", |ui| {
                if ui.button(None, format!("Move Left: {:?}", self.controls.left)) {
                    self.focused_on = Some(FocusedOn::Left);
                }
                if ui.button(None, format!("Move Right: {:?}", self.controls.right)) {
                    self.focused_on = Some(FocusedOn::Right);
                }
                if ui.button(None, format!("Soft Drop: {:?}", self.controls.soft_drop)) {
                    self.focused_on = Some(FocusedOn::SoftDrop);
                }
                if ui.button(None, format!("Hard Drop: {:?}", self.controls.hard_drop)) {
                    self.focused_on = Some(FocusedOn::HardDrop);
                }
                if ui.button(
                    None,
                    format!("Rotate Clockwise: {:?}", self.controls.rotate_clockwise),
                ) {
                    self.focused_on = Some(FocusedOn::RotateClockwise);
                }
                if ui.button(
                    None,
                    format!(
                        "Rotate Counterclockwise: {:?}",
                        self.controls.rotate_counterclockwise
                    ),
                ) {
                    self.focused_on = Some(FocusedOn::RotateCounterclockwise);
                }
                if ui.button(None, format!("Rotate 180: {:?}", self.controls.rotate_180)) {
                    self.focused_on = Some(FocusedOn::Rotate180);
                }
                if ui.button(None, format!("Hold: {:?}", self.controls.hold)) {
                    self.focused_on = Some(FocusedOn::Hold);
                }
                if ui.button(None, format!("Restart: {:?}", self.controls.restart)) {
                    self.focused_on = Some(FocusedOn::Restart);
                }
                if ui.button(None, format!("Undo: {:?}", self.controls.undo)) {
                    self.focused_on = Some(FocusedOn::Undo);
                }
                if ui.button(None, format!("Redo: {:?}", self.controls.redo)) {
                    self.focused_on = Some(FocusedOn::Redo);
                }
                if let Some(focused_on) = &self.focused_on {
                    if let Some(keycode) = get_last_key_pressed() {
                        match focused_on {
                            FocusedOn::Left => {
                                self.controls.left = keycode;
                                self.focused_on = None;
                            }
                            FocusedOn::Right => {
                                self.controls.right = keycode;
                                self.focused_on = None;
                            }
                            FocusedOn::SoftDrop => {
                                self.controls.soft_drop = keycode;
                                self.focused_on = None;
                            }
                            FocusedOn::HardDrop => {
                                self.controls.hard_drop = keycode;
                                self.focused_on = None;
                            }
                            FocusedOn::RotateClockwise => {
                                self.controls.rotate_clockwise = keycode;
                                self.focused_on = None;
                            }
                            FocusedOn::RotateCounterclockwise => {
                                self.controls.rotate_counterclockwise = keycode;
                                self.focused_on = None;
                            }
                            FocusedOn::Rotate180 => {
                                self.controls.rotate_180 = keycode;
                                self.focused_on = None;
                            }
                            FocusedOn::Hold => {
                                self.controls.hold = keycode;
                                self.focused_on = None;
                            }
                            FocusedOn::Restart => {
                                self.controls.restart = keycode;
                                self.focused_on = None
                            }
                            FocusedOn::Undo => {
                                self.controls.undo = keycode;
                            }
                            FocusedOn::Redo => {
                                self.controls.redo = keycode;
                            }
                        }
                    }
                }
            });
        });
        repeat_all_miniquad_input(self, self.subscriber_id);
    }
}

enum Mode {
    Zen,
    Bot,
}

impl Display for Mode {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Zen => write!(f, "{}", "Zen"),
            Self::Bot => write!(f, "{}", "Bot"),
        }
    }
}

impl Mode {
    pub fn new() -> Self {
        Self::Zen
    }

    pub async fn start(
        &self,
        bottom_left_corner: Cell<Vec2>,
        block_size: Cell<f32>,
        board: &mut Box<Board>,
        human: &mut Human,
        debug: Cell<bool>,
        open_settings: Cell<bool>,
    ) {
        let drawer = Drawer {
            bottom_left_corner: &bottom_left_corner,
            block_size: &block_size,
            debug: &debug,
        };

        match self {
            Self::Zen => {
                loop {
                    let block_size_temp = (screen_height() / (board.positions.len() as f32 * 1.25))
                        .min(screen_width() / (board.positions[0].len() as f32 * 1.25));
                    block_size.set(block_size_temp);
                    bottom_left_corner.set(crate::core::logic::consts::vec2(
                        block_size_temp * 6.0,
                        block_size_temp * (board.positions.len() as f32 + 2.0),
                    ));

                    if open_settings.get() {
                        human.input.settings.draw_menu();
                    } else {
                        match board.game_state {
                            State::Playing => {
                                clear_background(BLACK);

                                // modify board bricks
                                {
                                    let block_size = block_size.get();
                                    let bottom_left_corner = bottom_left_corner.get();
                                    let (x, y) = mouse_position();
                                    let x =
                                        ((x - bottom_left_corner.x) / block_size).floor() as usize;
                                    let y =
                                        ((bottom_left_corner.y - y) / block_size).floor() as usize;
                                    let brick = vec![crate::core::logic::consts::vec2(0.0, 0.0)];

                                    if is_mouse_button_down(MouseButton::Left)
                                        && !board.conflict(
                                            &brick,
                                            crate::core::logic::consts::vec2(x as f32, y as f32),
                                            false,
                                        )
                                    {
                                        board.add_brick(x, y, CUSTOM_GARBAGE);
                                    } else if is_mouse_button_down(MouseButton::Right)
                                        && !board.conflict(
                                            &brick,
                                            crate::core::logic::consts::vec2(x as f32, y as f32),
                                            false,
                                        )
                                    {
                                        board.remove_brick(x, y);
                                    }
                                }

                                human.input.handle(board);
                                drawer.draw_tetrominos(&board.positions);
                                drawer.draw_ghost_piece(&board);
                                drawer.draw_current_tetromino(&board.active_piece);
                                drawer.draw_preview_pieces(&board);
                                drawer.draw_hold_piece(&board);
                                if let Some(action) = &board.last_action {
                                    drawer.draw_action_text(&board, action.as_str());
                                }
                            }
                            State::GameOver => {
                                todo!();
                            }
                        }
                    }

                    // handle keyboard input
                    {
                        if is_key_pressed(KeyCode::Escape) {
                            match board.game_state {
                                State::Playing => {
                                    open_settings.set(!open_settings.get());
                                }
                                _ => unimplemented!(),
                            }
                        } else if is_key_pressed(KeyCode::F3) {
                            debug.set(!debug.get());
                        }
                    }
                    // handle_keyboard_input(&mut board, &debug, &mut open_settings);
                    next_frame().await;
                }
            }
            Self::Bot => {
                todo!();
            }
        }
    }
}
