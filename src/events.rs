use bevy::prelude::*;

#[derive(Event)]
pub struct DamageEvent(pub f32);

#[derive(Event)]
pub enum VacuumEvent {
    Start,
    Stop,
}

#[derive(Event)]
pub struct WaveEnd;

#[derive(Event)]
pub struct Sucked(pub Entity);

#[derive(Event)]
pub struct PickedUpgrade;

#[derive(Event)]
pub struct PlayerDied;