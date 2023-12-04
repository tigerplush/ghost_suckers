use bevy::prelude::*;

#[derive(Resource)]
pub struct InputValues {
    pub movement: Vec2,
}

impl InputValues {
    pub fn new() -> Self {
        Self {
            movement: Vec2::default(),
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
    pub fn new() -> Self {
        Self {
            translational_shake: 0.0,
            rotational_shake: 0.0,
            offset: Vec3::ZERO
        }
    }
}
