use egui::{Key, Modifiers};
use egui_bind::{Bind, KeyOrPointer};
use macroquad::prelude::KeyCode;

type Binding = Option<(KeyOrPointer, Modifiers)>;

// https://tetris.wiki/DAS
pub struct Settings {
    pub handles: Handles,
    pub controls: Controls,
    left: Binding,
    right: Binding,
    soft_drop: Binding,
    hard_drop: Binding,
    rotate_clockwise: Binding,
    rotate_counterclockwise: Binding,
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
            left: None,
            right: None,
            soft_drop: None,
            hard_drop: None,
            rotate_clockwise: None,
            rotate_counterclockwise: None,
        }
    }
}

impl Settings {
    pub fn draw_menu(&mut self) {
        egui_macroquad::ui(|egui_ctx| {
            egui::Window::new("Settings").show(egui_ctx, |ui| {
                ui.label("Handles");
                ui.add(egui::Slider::new(&mut self.handles.das, 0.0..=20.0).text("DAS (ms)"));
                ui.add(egui::Slider::new(&mut self.handles.arr, 0.0..=5.0).text("ARR (ms)"));

                ui.separator();
                ui.label("Controls");
                ui.separator();

                ui.label("Left");
                // let r = ui.checkbox(&mut self.left_text.check, "Left:");
                if ui.add(Bind::new("_left", &mut self.left)).changed() {
                    match self.left {
                        Some((KeyOrPointer::Key(key), _)) => {
                            self.controls.left = egui_key_to_macroquad_keycode(key);
                        }
                        _ => {}
                    }
                }
                ui.label("Right");
                if ui.add(Bind::new("_right", &mut self.right)).changed() {
                  match self.right {
                    Some((KeyOrPointer::Key(key), _)) => {
                        self.controls.right = egui_key_to_macroquad_keycode(key);
                    }
                    _ => {}
                  }
                }
                ui.label("Soft Drop");
                if ui
                    .add(Bind::new("_soft_drop", &mut self.soft_drop))
                    .changed()
                {
                  match self.soft_drop {
                    Some((KeyOrPointer::Key(key), _)) => {
                        self.controls.soft_drop = egui_key_to_macroquad_keycode(key);
                    }
                    _ => {}
                  }
                }
                ui.label("Hard Drop");
                if ui
                    .add(Bind::new("_hard_drop", &mut self.hard_drop))
                    .changed()
                {
                  match self.hard_drop {
                    Some((KeyOrPointer::Key(key), _)) => {
                        self.controls.hard_drop = egui_key_to_macroquad_keycode(key);
                    }
                    _ => {}
                  }
                }
                ui.label("Rotate Clockwise");
                if ui
                    .add(Bind::new("_rotate_clockwise", &mut self.rotate_clockwise))
                    .changed()
                {
                  match self.rotate_clockwise {
                    Some((KeyOrPointer::Key(key), _)) => {
                        self.controls.rotate_clockwise = egui_key_to_macroquad_keycode(key);
                    }
                    _ => {}
                  }
                }
                ui.label("Rotate Counter Clockwise");
                if ui
                    .add(Bind::new(
                        "_rotate_counterclockwise",
                        &mut self.rotate_counterclockwise,
                    ))
                    .changed()
                {
                  match self.rotate_counterclockwise {
                    Some((KeyOrPointer::Key(key), _)) => {
                        self.controls.rotate_counterclockwise = egui_key_to_macroquad_keycode(key);
                    }
                    _ => {}
                  }
                }
            });
        });

        egui_macroquad::draw();
    }
}

fn egui_key_to_macroquad_keycode(key: Key) -> KeyCode {
    match key {
        Key::ArrowDown => KeyCode::Down,
        Key::ArrowLeft => KeyCode::Left,
        Key::ArrowRight => KeyCode::Right,
        Key::ArrowUp => KeyCode::Up,
        Key::Escape => KeyCode::Escape,
        Key::Tab => KeyCode::Tab,
        Key::Backspace => KeyCode::Backspace,
        Key::Enter => KeyCode::Enter,
        Key::Space => KeyCode::Space,
        Key::Insert => KeyCode::Insert,
        Key::Delete => KeyCode::Delete,
        Key::Home => KeyCode::Home,
        Key::End => KeyCode::End,
        Key::PageUp => KeyCode::PageUp,
        Key::PageDown => KeyCode::PageDown,
        Key::Minus => KeyCode::Minus,
        Key::PlusEquals => todo!(),
        // Key::PlusEquals => KeyCode::PlusEquals,
        Key::Num0 => KeyCode::Key0,
        Key::Num1 => KeyCode::Key1,
        Key::Num2 => KeyCode::Key2,
        Key::Num3 => KeyCode::Key3,
        Key::Num4 => KeyCode::Key4,
        Key::Num5 => KeyCode::Key5,
        Key::Num6 => KeyCode::Key6,
        Key::Num7 => KeyCode::Key7,
        Key::Num8 => KeyCode::Key8,
        Key::Num9 => KeyCode::Key9,
        Key::A => KeyCode::A,
        Key::B => KeyCode::B,
        Key::C => KeyCode::C,
        Key::D => KeyCode::D,
        Key::E => KeyCode::E,
        Key::F => KeyCode::F,
        Key::G => KeyCode::G,
        Key::H => KeyCode::H,
        Key::I => KeyCode::I,
        Key::J => KeyCode::J,
        Key::K => KeyCode::K,
        Key::L => KeyCode::L,
        Key::M => KeyCode::M,
        Key::N => KeyCode::N,
        Key::O => KeyCode::O,
        Key::P => KeyCode::P,
        Key::Q => KeyCode::Q,
        Key::R => KeyCode::R,
        Key::S => KeyCode::S,
        Key::T => KeyCode::T,
        Key::U => KeyCode::U,
        Key::V => KeyCode::V,
        Key::W => KeyCode::W,
        Key::X => KeyCode::X,
        Key::Y => KeyCode::Y,
        Key::Z => KeyCode::Z,
        Key::F1 => KeyCode::F1,
        Key::F2 => KeyCode::F2,
        Key::F3 => KeyCode::F3,
        Key::F4 => KeyCode::F4,
        Key::F5 => KeyCode::F5,
        Key::F6 => KeyCode::F6,
        Key::F7 => KeyCode::F7,
        Key::F8 => KeyCode::F8,
        Key::F9 => KeyCode::F9,
        Key::F10 => KeyCode::F10,
        Key::F11 => KeyCode::F11,
        Key::F12 => KeyCode::F12,
        Key::F13 => KeyCode::F13,
        Key::F14 => KeyCode::F14,
        Key::F15 => KeyCode::F15,
        Key::F16 => KeyCode::F16,
        Key::F17 => KeyCode::F17,
        Key::F18 => KeyCode::F18,
        Key::F19 => KeyCode::F19,
        Key::F20 => KeyCode::F20,
    }
}
