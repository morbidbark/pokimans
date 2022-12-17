use bevy::prelude::*;

#[derive(Component)]
pub struct Speed(pub f32);

#[derive(Component)]
pub struct Target(pub Vec2);

#[derive(Bundle)]
pub struct PlayerBundle {
    pub speed: Speed,
    pub target: Target,
}
