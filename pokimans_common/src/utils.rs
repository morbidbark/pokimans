use bevy::prelude::*;


pub struct Vecs;
impl Vecs {
    pub const UP: Vec2 = Vec2::new(0.0, 1.0);
    pub const DOWN: Vec2 = Vec2::new(0.0, -1.0);
    pub const LEFT: Vec2 = Vec2::new(-1.0, 0.0);
    pub const RIGHT: Vec2 = Vec2::new(1.0, 0.0);
    pub const ZERO: Vec2 = Vec2::new(0.0, 0.0);
}
