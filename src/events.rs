use bevy::prelude::*;

#[derive(Event)]
pub struct DamageEvent(pub f32);

#[derive(Event)]
pub enum VacuumEvent {
    Start,
    Stop,
}