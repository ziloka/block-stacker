use bevy::prelude::Component;

#[derive(Component, Copy, Clone)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}
