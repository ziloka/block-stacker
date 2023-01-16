use egui::{Key, Modifiers};
use egui_bind::{Bind, KeyOrPointer};
use macroquad::{prelude::KeyCode, ui::hash};

type Binding = Option<(KeyOrPointer, Modifiers)>;

pub struct KeyInfo {
    pub key: KeyCode,
    binding: Binding,
}

// https://tetris.wiki/DAS
pub struct Settings {
    pub handles: Handles,
    pub controls: Controls,
}

pub struct Handles {
    pub das: f32, // Delayed Auto Shift in miliseconds
    pub arr: f32, // Auto Repeat Rate in miliseconds
}

pub struct Controls {
    pub left: KeyInfo,
    pub right: KeyInfo,
    pub soft_drop: KeyInfo,
    pub hard_drop: KeyInfo,
    pub rotate_clockwise: KeyInfo,
    pub rotate_counterclockwise: KeyInfo,
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
                left: KeyInfo {
                    key: KeyCode::Left,
                    binding: None,
                },
                right: KeyInfo {
                    key: KeyCode::Right,
                    binding: None,
                },
                soft_drop: KeyInfo {
                    key: KeyCode::Down,
                    binding: None,
                },
                hard_drop: KeyInfo {
                    key: KeyCode::Space,
                    binding: None,
                },
                rotate_clockwise: KeyInfo {
                    key: KeyCode::Up,
                    binding: None,
                },
                rotate_counterclockwise: KeyInfo {
                    key: KeyCode::Z,
                    binding: None,
                },
            },
        }
    }
}

impl Settings {
    pub fn draw_menu(&mut self) {
        /**
         * TODO: using macroquad::ui https://github.com/not-fl3/macroquad/blob/master/examples/ui.rs
         * TODO: reimplement it using macroquad's ui possibly using https://github.com/not-fl3/macroquad/blob/master/examples/events.rs
         * TODO: implement macroquad's ui for the settings menu via their buttons
         */
        egui_macroquad::ui(|egui_ctx| {
            egui::Window::new("Settings").show(egui_ctx, |ui| {
                ui.label("Handles");
                ui.add(egui::Slider::new(&mut self.handles.das, 0.0..=20.0).text("DAS (ms)"));
                ui.add(egui::Slider::new(&mut self.handles.arr, 0.0..=5.0).text("ARR (ms)"));

                ui.separator();
                ui.label("Controls");
                ui.separator();

                ui.label("Left");
                Settings::read_keybinding(
                    ui.add(Bind::new(hash!(), &mut self.controls.left.binding)),
                    &mut self.controls.left,
                );
                ui.label("Right");
                Settings::read_keybinding(
                    ui.add(Bind::new(hash!(), &mut self.controls.right.binding)),
                    &mut self.controls.right,
                );
                ui.label("Soft Drop");
                Settings::read_keybinding(
                    ui.add(Bind::new(hash!(), &mut self.controls.soft_drop.binding)),
                    &mut self.controls.soft_drop,
                );
                ui.label("Hard Drop");
                Settings::read_keybinding(
                    ui.add(Bind::new(hash!(), &mut self.controls.hard_drop.binding)),
                    &mut self.controls.hard_drop,
                );
                ui.label("Rotate Clockwise");
                Settings::read_keybinding(
                    ui.add(Bind::new(
                        hash!(),
                        &mut self.controls.rotate_clockwise.binding,
                    )),
                    &mut self.controls.rotate_clockwise,
                );
                ui.label("Rotate Counter Clockwise");
                Settings::read_keybinding(
                    ui.add(Bind::new(
                        hash!(),
                        &mut self.controls.rotate_counterclockwise.binding,
                    )),
                    &mut self.controls.rotate_counterclockwise,
                );
            });
        });

        egui_macroquad::draw();
    }

    fn read_keybinding(response: egui::Response, key_info: &mut KeyInfo) {
        if response.changed() {
            match key_info.binding {
                Some((KeyOrPointer::Key(egui_key), _)) => {
                    key_info.key = egui_key_to_macroquad_keycode(egui_key);
                }
                _ => {}
            }
        }
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
