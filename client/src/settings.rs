use macroquad::{
    miniquad::{graphics::GraphicsContext, EventHandler, KeyMods, TouchPhase},
    prelude::{
        get_last_key_pressed,
        utils::{register_input_subscriber, repeat_all_miniquad_input},
        vec2, KeyCode, MouseButton,
    },
    ui::{hash, root_ui, widgets::Window},
};

// https://tetris.wiki/DAS
pub struct Settings {
    pub handles: Handles,
    pub controls: Controls,
    focused_on: Option<FocusedOn>,
    subscriber_id: usize,
}

// https://www.reddit.com/r/Tetris/comments/frbii6/comment/fphx9ml?context=3
// https://www.reddit.com/r/Tetris/comments/13uqby/comment/c77ev43/?context=3
pub struct Handles {
    pub das: f32, // Delayed Auto Shift in frames
    pub arr: f32, // Auto Repeat Rate in frames
}

pub struct Controls {
    pub left: KeyCode,
    pub right: KeyCode,
    pub soft_drop: KeyCode,
    pub hard_drop: KeyCode,
    pub rotate_clockwise: KeyCode,
    pub rotate_counterclockwise: KeyCode,
    pub hold: KeyCode,
    pub restart: KeyCode,
}

enum FocusedOn {
    Left,
    Right,
    SoftDrop,
    HardDrop,
    RotateClockwise,
    RotateCounterclockwise,
    Hold,
    Restart,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            handles: Handles { das: 8.6, arr: 0.6 },
            controls: Controls {
                left: KeyCode::Left,
                right: KeyCode::Right,
                soft_drop: KeyCode::Down,
                hard_drop: KeyCode::Space,
                rotate_clockwise: KeyCode::Up,
                rotate_counterclockwise: KeyCode::Z,
                hold: KeyCode::C,
                restart: KeyCode::R,
            },
            focused_on: None,
            subscriber_id: register_input_subscriber(),
        }
    }
}

impl Settings {
    pub fn draw_menu(&mut self) {
        Window::new(hash!(), vec2(0., 0.), vec2(300., 800.))
            .label("Settings")
            .titlebar(true)
            .ui(&mut root_ui(), |ui| {
                ui.tree_node(hash!(), "handles", |ui| {
                    ui.slider(hash!(), "DAS (frames)", 0.0..200.0, &mut self.handles.das);
                    ui.slider(hash!(), "ARR (frames)", 0.0..50., &mut self.handles.arr);
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
                    if ui.button(None, format!("Hold: {:?}", self.controls.hold)) {
                        self.focused_on = Some(FocusedOn::Hold);
                    }
                    if ui.button(None, format!("Restart: {:?}", self.controls.restart)) {
                        self.focused_on = Some(FocusedOn::Restart);
                    }
                    if let Some(focused_on) = &self.focused_on {
                        if let Some(keycode) = get_last_key_pressed() {
                            match focused_on {
                                FocusedOn::Left => self.controls.left = keycode,
                                FocusedOn::Right => self.controls.right = keycode,
                                FocusedOn::SoftDrop => self.controls.soft_drop = keycode,
                                FocusedOn::HardDrop => self.controls.hard_drop = keycode,
                                FocusedOn::RotateClockwise => {
                                    self.controls.rotate_clockwise = keycode
                                }
                                FocusedOn::RotateCounterclockwise => {
                                    self.controls.rotate_counterclockwise = keycode
                                }
                                FocusedOn::Hold => self.controls.hold = keycode,
                                FocusedOn::Restart => self.controls.restart = keycode,
                            }
                            self.focused_on = None;
                        }
                    }
                });
            });
        repeat_all_miniquad_input(self, self.subscriber_id);
    }
}

impl EventHandler for Settings {
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
        // println!("mouse motion event");
    }
    fn mouse_wheel_event(&mut self, _ctx: &mut GraphicsContext, _x: f32, _y: f32) {
        // println!("mouse wheel event");
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
