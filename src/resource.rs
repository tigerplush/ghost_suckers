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
    pub falloff: f32,
}

impl CameraSettings {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            translational_shake: 0.0,
            translational_strength: 1.0,
            rotational_shake: 0.0,
            offset: Vec3::ZERO,
            falloff: 1.1,
        }
    }

    pub fn tick(&mut self, time: Duration) {
        self.translational_shake = (self.translational_shake / self.falloff - time.as_secs_f32()).max(0.0);
        self.rotational_shake = (self.rotational_shake / self.falloff - time.as_secs_f32()).max(0.0);
    }

    pub fn add(&mut self, value: f32) {
        self.translational_shake += value;
        self.rotational_shake += value;
    }
}

#[derive(Debug, Resource)]
pub struct Stats {
    pub max_health: f32,
    pub health: f32,
    pub regeneration: f32,
    pub sucked_ghosts: u32,
    pub suck_time: f32,
    pub movement_speed: f32,
    pub reg_paused: bool,
}

impl Stats {
    pub fn new() -> Self {
        Self {
            max_health: 100.0,
            health: 100.0,
            regeneration: 1.0,
            sucked_ghosts: 0,
            suck_time: 0.5,
            movement_speed: 5.0,
            reg_paused: false,
        }
    }

    pub fn normalized_health(&self) -> f32 {
        self.health / self.max_health
    }

    pub fn regenerate(&mut self, value: f32) {
        self.health = (self.health + value * self.regeneration).clamp(0.0, self.max_health);
    }

    pub fn add_health_percent(&mut self, value: f32) {
        self.health = (self.health + self.max_health * value).clamp(0.0, self.max_health);
    }
}