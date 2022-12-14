use crate::math::Plane;
use bevy::prelude::*;

#[derive(Component)]
pub struct Player {
    pub plane: Plane,
}

impl Player {
    pub fn new() -> Self {
        Self {
            plane: Plane {
                origin: Vec3::ZERO,
                normal: Vec3::new(0.0, 0.0, 1.0),
            },
        }
    }
}
