use bevy::prelude::*;

#[derive(Resource)]
pub struct InputValues {
    pub movement: Vec2,
    pub mouse_pressed: bool,
    pub mouse_position: Vec3,
}

impl InputValues {
    pub fn new() -> Self {
        Self {
            movement: Vec2::default(),
            mouse_pressed: false,
            mouse_position: Vec3::default()
        }
    }
}

#[derive(Default, Resource)]
pub struct CameraSettings {
    pub translational_shake: f32,
    pub rotational_shake: f32,
    pub offset: Vec3,
}

impl CameraSettings {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            translational_shake: 0.0,
            rotational_shake: 0.0,
            offset: Vec3::ZERO
        }
    }
}

#[derive(Resource)]
pub struct Stats {
    pub health: f32,
}

impl Stats {
    pub fn new() -> Self {
        Self {
            health: 100.0,
        }
    }
}