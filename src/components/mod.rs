mod player;

use bevy::prelude::Component;

pub use player::Player;

#[derive(Component)]
pub struct Speed(pub f32);