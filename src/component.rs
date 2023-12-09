use bevy::{prelude::*, time::Stopwatch};

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

#[derive(Deref, DerefMut, Component)]
pub struct FloatTimer(pub Stopwatch);

#[derive(Deref, DerefMut, Component)]
pub struct SuckTimer(pub Timer);

impl FloatTimer {
    pub fn new() -> Self {
        Self(Stopwatch::new())
    }
}