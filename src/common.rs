use bevy::math::Vec3;
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
