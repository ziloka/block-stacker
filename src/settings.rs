use macroquad::{
    prelude::{get_last_key_pressed, vec2, KeyCode},
    ui::{hash, root_ui, widgets::Window},
};

enum FocusedOn {
    Left,
    Right,
    SoftDrop,
    HardDrop,
    RotateClockwise,
    RotateCounterclockwise,
}

// https://tetris.wiki/DAS
pub struct Settings {
    pub handles: Handles,
    pub controls: Controls,
    focused_on: Option<FocusedOn>,
}

pub struct Handles {
    pub das: f32, // Delayed Auto Shift in miliseconds
    pub arr: f32, // Auto Repeat Rate in miliseconds
}

pub struct Controls {
    pub left: KeyCode,
    pub right: KeyCode,
    pub soft_drop: KeyCode,
    pub hard_drop: KeyCode,
    pub rotate_clockwise: KeyCode,
    pub rotate_counterclockwise: KeyCode,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            // https://www.reddit.com/r/Tetris/comments/frbii6/jstris_das_arr_in_tetrio/
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
            },
            focused_on: None,
        }
    }
}

impl Settings {
    pub fn draw_menu(&mut self) {
        Window::new(hash!(), vec2(0., 0.), vec2(300., 800.))
            .label("Settings")
            .titlebar(true)
            .ui(&mut *root_ui(), |ui| {
                ui.tree_node(hash!(), "handles", |ui| {
                    ui.slider(hash!(), "DAS (frames)", 20.0..0., &mut self.handles.das);
                    ui.slider(hash!(), "ARR (frames)", 0.0..5., &mut self.handles.arr);
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
                            }
                            self.focused_on = None;
                        }
                    }
                });
            });
    }
}
