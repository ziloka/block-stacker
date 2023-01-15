use macroquad::{
    prelude::{vec2, KeyCode},
    ui::{hash, root_ui, widgets::Window},
};

// https://tetris.wiki/DAS
pub struct Settings {
    pub handles: Handles,
    pub controls: Controls,
    left_text: String,
    right_text: String,
    soft_drop_text: String,
    hard_drop_text: String,
    rotate_clockwise_text: String,
    rotate_counterclockwise_text: String,
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
            left_text: "".to_string(),
            right_text: "".to_string(),
            soft_drop_text: "".to_string(),
            hard_drop_text: "".to_string(),
            rotate_clockwise_text: "".to_string(),
            rotate_counterclockwise_text: "".to_string(),
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
                    ui.input_text(hash!(), "Left:", &mut self.left_text);
                    ui.input_text(hash!(), "Right:", &mut self.right_text);
                    ui.input_text(hash!(), "Soft Drop: {:?}", &mut self.soft_drop_text);
                    ui.input_text(hash!(), "Hard Drop: {:?}", &mut self.hard_drop_text);
                    ui.input_text(
                        hash!(),
                        "Rotate Clockwise: {:?}",
                        &mut self.rotate_clockwise_text,
                    );
                    ui.input_text(
                        hash!(),
                        "Rotate Counterclockwise: {:?}",
                        &mut self.rotate_counterclockwise_text,
                    );
                });
            });
    }
}
