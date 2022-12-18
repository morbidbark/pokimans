use bevy::prelude::*;

use crate::input::InputHandler;


pub fn setup_camera(mut commands: Commands) {
    let mut camera = Camera2dBundle::default();
    camera.projection.scaling_mode = bevy::render::camera::ScalingMode::Auto {
	min_width: 16.0,
	min_height: 8.0,
    };
    commands.spawn(camera);
}

pub fn move_camera(
    player: Query<&Transform, With<InputHandler>>,
    mut camera: Query<&mut Transform, (With<Camera2d>, Without<InputHandler>)>,
) {
    camera.get_single_mut().unwrap().translation = player.get_single().unwrap().translation; 
}
