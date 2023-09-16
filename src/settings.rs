use macroquad::{
    miniquad::{EventHandler, KeyMods},
    prelude::{
        get_last_key_pressed,
        utils::{register_input_subscriber, repeat_all_miniquad_input},
        vec2, KeyCode,
    },
    ui::{hash, root_ui, widgets::Window},
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
                rotate_counterclockwise: KeyCode::Z,
                rotate_180: KeyCode::A,
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
                                FocusedOn::Rotate180 => self.controls.rotate_180 = keycode,
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
    fn update(&mut self) {}
    fn draw(&mut self) {}

    fn char_event(&mut self, _character: char, _keymods: KeyMods, _repeat: bool) {}
}
