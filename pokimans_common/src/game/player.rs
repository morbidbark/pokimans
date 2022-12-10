use bevy::prelude::*;
use bevy::render::color::Color;
use crate::utils::Position;

#[derive(Component)]
pub struct Player {
    pub name: String,
    pub position: Position,
    pub color: Color,
}
    
