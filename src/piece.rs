use bevy::prelude::Component;

#[derive(Component, Copy, Clone, Debug)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}
