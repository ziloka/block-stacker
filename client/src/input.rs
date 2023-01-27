use std::default;

use macroquad::{
    prelude::{is_key_down, is_key_pressed, MouseButton, KeyCode, utils::{repeat_all_miniquad_input, register_input_subscriber}},
    miniquad::{graphics::GraphicsContext, EventHandler, KeyMods, TouchPhase, date::now},
};

use tetris::{
    board::Board,
    consts::{vec2, HEIGHT},
};

use crate::settings::Settings;

pub struct Input {
    pub settings: Settings,
    holding: bool,
    frame: f64, // the time the last movement was made
    subscriber_id: usize,
}

impl default::Default for Input {
    fn default() -> Self {
        Self {
            settings: Settings::default(),
            holding: false,
            frame: 0.0,
            subscriber_id: register_input_subscriber()
        }
    }
}

impl Input {
    // handles movement for every frame
    pub fn handle(&mut self, board: &mut Board) {
        self.holding = is_key_down(self.settings.controls.left)
            || is_key_down(self.settings.controls.right)
            || is_key_down(self.settings.controls.soft_drop);

        let das = self.settings.handles.das as f64;
        let arr = self.settings.handles.arr as f64;
        let frame = self.frame;

        if (!self.holding && frame % das <= das) || (self.holding && frame % arr <= arr) {
            if is_key_down(self.settings.controls.left) && !board.conflict(vec2(-1.0, 0.0)) {
                for dot in board.active_piece.dots.iter_mut() {
                    dot.x -= 1.0;
                }
            } else if is_key_down(self.settings.controls.right) && !board.conflict(vec2(1.0, 0.0)) {
                for dot in board.active_piece.dots.iter_mut() {
                    dot.x += 1.0;
                }
            } else if is_key_down(self.settings.controls.soft_drop)
                && !board.conflict(vec2(0.0, 1.0))
            {
                for dot in board.active_piece.dots.iter_mut() {
                    dot.y += 1.0;
                }
            }
        }

        // https://github.com/JohnnyTurbo/LD43/blob/82de0ac5aa29f6e87d6c5417e0504d6ae7033ef6/Assets/Scripts/PiecesController.cs#L140-L147
        if is_key_pressed(self.settings.controls.rotate_clockwise) {
            board.rotate_tetromino(true, true); // rotate clockwise
        } else if is_key_pressed(self.settings.controls.rotate_counterclockwise) {
            board.rotate_tetromino(false, true); // rotate clockwise
        } else if is_key_pressed(self.settings.controls.hard_drop) {
            // the hard drop
            let mut y_offset = 0.0;
            for y in 0..HEIGHT as i32 {
                let y = y as f32;
                if !board.conflict(vec2(0.0, y)) {
                    y_offset = y;
                }
            }
            for dot in board.active_piece.dots.iter_mut() {
                dot.y += y_offset;
            }
            board.set_active_tetromino_position();
        } else if is_key_pressed(self.settings.controls.hold) {
            board.hold_tetromino();
        } else if is_key_pressed(self.settings.controls.restart) {
            *board = Board::new(now() as usize);
        }

        // handle line clears
        board.clear_lines();

        repeat_all_miniquad_input(self, self.subscriber_id);

        // update frame
        self.frame = self.frame + 1.0;
    }
}

impl EventHandler for Input {
    fn update(&mut self, _ctx: &mut GraphicsContext) {
        println!("update");
    }
    fn draw(&mut self, _ctx: &mut GraphicsContext) {
        println!("draw");
    }

    fn resize_event(&mut self, _ctx: &mut GraphicsContext, _width: f32, _height: f32) {
        println!("resize event");
    }
    fn mouse_motion_event(&mut self, _ctx: &mut GraphicsContext, _x: f32, _y: f32) {
        println!("mouse motion event");
    }
    fn mouse_wheel_event(&mut self, _ctx: &mut GraphicsContext, _x: f32, _y: f32) {
        println!("mouse wheel event");
    }
    fn mouse_button_down_event(
        &mut self,
        _ctx: &mut GraphicsContext,
        _button: MouseButton,
        _x: f32,
        _y: f32,
    ) {
        println!("mouse button down event");
    }
    fn mouse_button_up_event(
        &mut self,
        _ctx: &mut GraphicsContext,
        _button: MouseButton,
        _x: f32,
        _y: f32,
    ) {
        println!("mouse button up event");
    }

    fn char_event(
        &mut self,
        _ctx: &mut GraphicsContext,
        _character: char,
        _keymods: KeyMods,
        _repeat: bool,
    ) {
        println!("char event");
    }

    fn key_down_event(
        &mut self,
        _ctx: &mut GraphicsContext,
        _keycode: KeyCode,
        _keymods: KeyMods,
        _repeat: bool,
    ) {
        println!("key down event");
    }

    fn key_up_event(&mut self, _ctx: &mut GraphicsContext, _keycode: KeyCode, _keymods: KeyMods) {
        println!("key up event");
    }

    /// Default implementation emulates mouse clicks
    fn touch_event(
        &mut self,
        ctx: &mut GraphicsContext,
        phase: TouchPhase,
        _id: u64,
        x: f32,
        y: f32,
    ) {
        println!("touch event");
        if phase == TouchPhase::Started {
            self.mouse_button_down_event(ctx, MouseButton::Left, x, y);
        }

        if phase == TouchPhase::Ended {
            self.mouse_button_up_event(ctx, MouseButton::Left, x, y);
        }

        if phase == TouchPhase::Moved {
            self.mouse_motion_event(ctx, x, y);
        }
    }

    /// Represents raw hardware mouse motion event
    /// Note that these events are delivered regardless of input focus and not in pixels, but in
    /// hardware units instead. And those units may be different from pixels depending on the target platform
    fn raw_mouse_motion(&mut self, _ctx: &mut GraphicsContext, _dx: f32, _dy: f32) {
        println!("raw mouse motion");
    }

    /// Window has been minimized
    /// Right now is only implemented on Android, and is called on a Pause ndk callback
    fn window_minimized_event(&mut self, _ctx: &mut GraphicsContext) {
        println!("window minimized event");
    }

    /// Window has been restored
    /// Right now is only implemented on Android, and is called on a Resume ndk callback
    fn window_restored_event(&mut self, _ctx: &mut GraphicsContext) {
        println!("window restored event");
    }

    /// This event is sent when the userclicks the window's close button
    /// or application code calls the ctx.request_quit() function. The event
    /// handler callback code can handle this event by calling
    /// ctx.cancel_quit() to cancel the quit.
    /// If the event is ignored, the application will quit as usual.
    fn quit_requested_event(&mut self, _ctx: &mut GraphicsContext) {
        println!("quit requested event");
    }

    /// A file has been dropped over the application.
    /// Applications can request the number of dropped files with
    /// `ctx.dropped_file_count()`, path of an individual file with
    /// `ctx.dropped_file_path()`, and for wasm targets the file bytes
    /// can be requested with `ctx.dropped_file_bytes()`.
    fn files_dropped_event(&mut self, _ctx: &mut GraphicsContext) {
        println!("files dropped event");
    }
}
