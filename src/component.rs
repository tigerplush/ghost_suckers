use std::time::Duration;

use bevy::{prelude::*, time::Stopwatch};

use crate::common::Remap;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct FollowCamera;

#[derive(Component)]
pub struct Ghost(pub f32);

#[derive(Component)]
pub struct Suckable;

#[derive(Component)]
pub struct Nozzle;

#[derive(Component)]
pub struct Damage(pub f32);

#[derive(Component)]
pub struct FloatTimer {
    timer: Stopwatch,
    height: (f32, f32)
}

#[derive(Deref, DerefMut, Component)]
pub struct SuckTimer(pub Timer);

impl FloatTimer {
    pub fn new(height: (f32, f32)) -> Self {
        Self {
            timer: Stopwatch::new(),
            height,
        }
    }
    
    pub fn tick(&mut self, delta: Duration) {
        self.timer.tick(delta);
    }

    pub fn height(&self) -> f32 {
        self.timer
            .elapsed_secs()
            .sin()
            .remap((-1.0, 1.0), self.height)
    }
}