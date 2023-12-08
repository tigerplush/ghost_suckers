use std::f32::consts::PI;

use bevy::prelude::*;
use rand::prelude::*;

pub trait Remap<T = Self> {
    fn remap(&self, from: (T, T), to: (T, T)) -> Self;
}

impl Remap for f32 {
    fn remap(&self, from: (Self, Self), to: (Self, Self)) -> Self {
        to.0 + (self - from.0) * (to.1 - to.0) / (from.1 - from.0)
    }
}

pub trait Random {
    fn random() -> Self;
}

impl Random for Vec3 {
    fn random() -> Self {
        let mut rng = rand::thread_rng();
        Self {
            x: rng.gen(),
            y: rng.gen(),
            z: rng.gen(),
        }
    }
}

impl Random for Quat {
    fn random() -> Self {
        let mut rng = rand::thread_rng();
        Self::from_euler(bevy::math::EulerRot::XYZ, rng.gen(), rng.gen(), rng.gen())
    }
}

pub fn point_in_circle(radius: f32) -> (f32, f32) {
    let mut rng = rand::thread_rng();
    let angle = rng.gen_range(0.0..=PI*2.0);
    let random_radius = rng.gen_range(0.0..radius);
    let sin = angle.sin() * random_radius;
    let cos = angle.cos() * random_radius;
    (sin, cos)
}