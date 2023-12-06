use std::time::Duration;

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
    pub translational_strength: f32,
    pub rotational_shake: f32,
    pub offset: Vec3,
    pub camera_shake: f32,
}

impl CameraSettings {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            translational_shake: 0.0,
            translational_strength: 1.0,
            rotational_shake: 0.0,
            offset: Vec3::ZERO,
            camera_shake: 0.0,
        }
    }

    pub fn tick(&mut self, time: Duration) {
        self.translational_shake = (self.translational_shake - time.as_secs_f32()).max(0.0);
        self.rotational_shake = (self.rotational_shake - time.as_secs_f32()).max(0.0);
    }

    pub fn add(&mut self, value: f32) {
        self.translational_shake += value;
        self.rotational_shake += value;
    }
}

#[derive(Resource)]
pub struct Stats {
    pub health: f32,
    pub sucked_ghosts: u32,
}

impl Stats {
    pub fn new() -> Self {
        Self {
            health: 100.0,
            sucked_ghosts: 0,
        }
    }
}