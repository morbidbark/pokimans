use bevy::prelude::*;

#[derive(Component)]
pub struct Speed(pub f32);

#[derive(Component)]
pub struct Direction(pub Vec3);

#[derive(Bundle)]
pub struct PlayerBundle {
    pub speed: Speed,
    pub	direction: Direction,
}
