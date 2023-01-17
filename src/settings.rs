use macroquad::{
    prelude::{vec2, KeyCode, get_last_key_pressed},
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
                    if ui.button(
                        None,
                        format!("Left: {}", map_keycode_to_string(self.controls.left)),
                    ) {
                        self.focused_on = Some(FocusedOn::Left);
                    }
                    if ui.button(None, format!("Right: {}", map_keycode_to_string(self.controls.right))) {
                        self.focused_on = Some(FocusedOn::Right);
                    }
                    if ui.button(None,  format!("Soft Drop: {}", map_keycode_to_string(self.controls.soft_drop))) {
                        self.focused_on = Some(FocusedOn::SoftDrop);
                    }
                    if ui.button(None, format!("Hard Drop: {}", map_keycode_to_string(self.controls.hard_drop))) {
                        self.focused_on = Some(FocusedOn::HardDrop);
                    }
                    if ui.button(None, format!("Rotate Clockwise: {}", map_keycode_to_string(self.controls.rotate_clockwise))) {
                        self.focused_on = Some(FocusedOn::RotateClockwise);
                    }
                    if ui.button(None, format!("Rotate Counterclockwise: {}", map_keycode_to_string(self.controls.rotate_counterclockwise))) {
                        self.focused_on = Some(FocusedOn::RotateCounterclockwise);
                    }

                    if let Some(focused_on) = &self.focused_on {
                        if let Some(keycode) = get_last_key_pressed() {
                            match focused_on {
                                FocusedOn::Left => self.controls.left = keycode,
                                FocusedOn::Right => self.controls.right = keycode,
                                FocusedOn::SoftDrop => self.controls.soft_drop = keycode,
                                FocusedOn::HardDrop => self.controls.hard_drop = keycode,
                                FocusedOn::RotateClockwise => self.controls.rotate_clockwise = keycode,
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

fn map_keycode_to_string(keycode: KeyCode) -> String {
    match keycode {
        KeyCode::Space => "Space".to_string(),
        KeyCode::Apostrophe => "Apostrophe".to_string(),
        KeyCode::Comma => "Comma".to_string(),
        KeyCode::Minus => "Minus".to_string(),
        KeyCode::Period => "Period".to_string(),
        KeyCode::Slash => "Slash".to_string(),
        KeyCode::Key0 => "Key0".to_string(),
        KeyCode::Key1 => "Key1".to_string(),
        KeyCode::Key2 => "Key2".to_string(),
        KeyCode::Key3 => "Key3".to_string(),
        KeyCode::Key4 => "Key4".to_string(),
        KeyCode::Key5 => "Key5".to_string(),
        KeyCode::Key6 => "Key6".to_string(),
        KeyCode::Key7 => "Key7".to_string(),
        KeyCode::Key8 => "Key8".to_string(),
        KeyCode::Key9 => "Key9".to_string(),
        KeyCode::Semicolon => "Semicolon".to_string(),
        KeyCode::Equal => "Equal".to_string(),
        KeyCode::A => "A".to_string(),
        KeyCode::B => "B".to_string(),
        KeyCode::C => "C".to_string(),
        KeyCode::D => "D".to_string(),
        KeyCode::E => "E".to_string(),
        KeyCode::F => "F".to_string(),
        KeyCode::G => "G".to_string(),
        KeyCode::H => "H".to_string(),
        KeyCode::I => "I".to_string(),
        KeyCode::J => "J".to_string(),
        KeyCode::K => "K".to_string(),
        KeyCode::L => "L".to_string(),
        KeyCode::M => "M".to_string(),
        KeyCode::N => "N".to_string(),
        KeyCode::O => "O".to_string(),
        KeyCode::P => "P".to_string(),
        KeyCode::Q => "Q".to_string(),
        KeyCode::R => "R".to_string(),
        KeyCode::S => "S".to_string(),
        KeyCode::T => "T".to_string(),
        KeyCode::U => "U".to_string(),
        KeyCode::V => "V".to_string(),
        KeyCode::W => "W".to_string(),
        KeyCode::X => "X".to_string(),
        KeyCode::Y => "Y".to_string(),
        KeyCode::Z => "Z".to_string(),
        KeyCode::LeftBracket => "LeftBracket".to_string(),
        KeyCode::Backslash => "Backslash".to_string(),
        KeyCode::RightBracket => "RightBracket".to_string(),
        KeyCode::GraveAccent => "GraveAccent".to_string(),
        KeyCode::World1 => "World1".to_string(),
        KeyCode::World2 => "World2".to_string(),
        KeyCode::Escape => "Escape".to_string(),
        KeyCode::Enter => "Enter".to_string(),
        KeyCode::Tab => "Tab".to_string(),
        KeyCode::Backspace => "Backspace".to_string(),
        KeyCode::Insert => "Insert".to_string(),
        KeyCode::Delete => "Delete".to_string(),
        KeyCode::Right => "Right".to_string(),
        KeyCode::Left => "Left".to_string(),
        KeyCode::Down => "Down".to_string(),
        KeyCode::Up => "Up".to_string(),
        KeyCode::PageUp => "PageUp".to_string(),
        KeyCode::PageDown => "PageDown".to_string(),
        KeyCode::Home => "Home".to_string(),
        KeyCode::End => "End".to_string(),
        KeyCode::CapsLock => "CapsLock".to_string(),
        KeyCode::ScrollLock => "ScrollLock".to_string(),
        KeyCode::NumLock => "NumLock".to_string(),
        KeyCode::PrintScreen => "PrintScreen".to_string(),
        KeyCode::Pause => "Pause".to_string(),
        KeyCode::F1 => "F1".to_string(),
        KeyCode::F2 => "F2".to_string(),
        KeyCode::F3 => "F3".to_string(),
        KeyCode::F4 => "F4".to_string(),
        KeyCode::F5 => "F5".to_string(),
        KeyCode::F6 => "F6".to_string(),
        KeyCode::F7 => "F7".to_string(),
        KeyCode::F8 => "F8".to_string(),
        KeyCode::F9 => "F9".to_string(),
        KeyCode::F10 => "F10".to_string(),
        KeyCode::F11 => "F11".to_string(),
        KeyCode::F12 => "F12".to_string(),
        KeyCode::F13 => "F13".to_string(),
        KeyCode::F14 => "F14".to_string(),
        KeyCode::F15 => "F15".to_string(),
        KeyCode::F16 => "F16".to_string(),
        KeyCode::F17 => "F17".to_string(),
        KeyCode::F18 => "F18".to_string(),
        KeyCode::F19 => "F19".to_string(),
        KeyCode::F20 => "F20".to_string(),
        KeyCode::F21 => "F21".to_string(),
        KeyCode::F22 => "F22".to_string(),
        KeyCode::F23 => "F23".to_string(),
        KeyCode::F24 => "F24".to_string(),
        KeyCode::F25 => "F25".to_string(),
        KeyCode::Kp0 => "Kp0".to_string(),
        KeyCode::Kp1 => "Kp1".to_string(),
        KeyCode::Kp2 => "Kp2".to_string(),
        KeyCode::Kp3 => "Kp3".to_string(),
        KeyCode::Kp4 => "Kp4".to_string(),
        KeyCode::Kp5 => "Kp5".to_string(),
        KeyCode::Kp6 => "Kp6".to_string(),
        KeyCode::Kp7 => "Kp7".to_string(),
        KeyCode::Kp8 => "Kp8".to_string(),
        KeyCode::Kp9 => "Kp9".to_string(),
        KeyCode::KpDecimal => "KpDecimal".to_string(),
        KeyCode::KpDivide => "KpDivide".to_string(),
        KeyCode::KpMultiply => "KpMultiply".to_string(),
        KeyCode::KpSubtract => "KpSubtract".to_string(),
        KeyCode::KpAdd => "KpAdd".to_string(),
        KeyCode::KpEnter => "KpEnter".to_string(),
        KeyCode::KpEqual => "KpEqual".to_string(),
        KeyCode::LeftShift => "LeftShift".to_string(),
        KeyCode::LeftControl => "LeftControl".to_string(),
        KeyCode::LeftAlt => "LeftAlt".to_string(),
        KeyCode::LeftSuper => "LeftSuper".to_string(),
        KeyCode::RightShift => "RightShift".to_string(),
        KeyCode::RightControl => "RightControl".to_string(),
        KeyCode::RightAlt => "RightAlt".to_string(),
        KeyCode::RightSuper => "RightSuper".to_string(),
        KeyCode::Menu => "Menu".to_string(),
        KeyCode::Unknown => "Unknown".to_string(),
    }
}
