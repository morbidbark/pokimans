use bevy::prelude::*;
use pokimans_common::movement::{MoveEvent, MovementControllerBundle};

use crate::{
    camera::{
	move_camera,
	setup_camera
    },
    input::{handle_input, InputHandler},
};


fn setup_player(mut commands: Commands, assets: Res<AssetServer>) {
    commands.spawn((
	assets.load::<Image, &str>("player.png"),
	Sprite {
		anchor: bevy::sprite::Anchor::BottomLeft,
		custom_size: Some(Vec2::new(1.0, 2.0)),
		..default()
	},
	InputHandler,
	MovementControllerBundle::new(),
    ));
}
    
pub struct PlayerPlugin;

impl bevy::app::Plugin for PlayerPlugin {
    fn build(&self, app: &mut bevy::app::App) {
	app
	    .add_event::<MoveEvent>()
	    .add_startup_system(setup_player)
	    .add_startup_system(setup_camera)
	    .add_system(handle_input)
	    .add_system(move_camera);
    }
}
