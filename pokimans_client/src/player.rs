use bevy::prelude::*;
use pokimans_common::game::player;
use pokimans_common::game::map;

const UP: Vec2 = Vec2::new(0.0, 1.0);
const DOWN: Vec2 = Vec2::new(0.0, -1.0);
const LEFT: Vec2 = Vec2::new(-1.0, 0.0);
const RIGHT: Vec2 = Vec2::new(1.0, 0.0);
const ZERO: Vec2 = Vec2::new(0.0, 0.0);


#[derive(Component)]
pub struct Controller;
pub fn handle_input(
    time: Res<Time>,
    input: Res<Input<KeyCode>>,
    map: Res<map::Map>,
    mut player_props: Query<(&mut player::Speed, &mut Transform, &mut player::Target), With<Controller>>,
    mut camera: Query<&mut Transform, (With<Camera2d>, Without<Controller>)>,
) {

    let (speed, mut transform, mut target) = player_props.get_single_mut().unwrap();

    let target_direction = target.0 - transform.translation.truncate();
    if target_direction.length() > 0.01 {
	transform.translation += target_direction.extend(0.0).normalize() * time.delta_seconds() * speed.0;
	camera.get_single_mut().unwrap().translation = transform.translation; 
	return;
    }
 
    // Determine player target direction
    let mut new_direction = ZERO;
    if input.pressed(KeyCode::W) { new_direction += UP }
    else if input.pressed(KeyCode::A) { new_direction += LEFT }
    else if input.pressed(KeyCode::S) { new_direction += DOWN }
    else if input.pressed(KeyCode::D) { new_direction += RIGHT }

    let new_target = transform.translation.round().truncate() + new_direction;

    // Determine if the target location is traversible
    let chunk = map.chunks.get(0).unwrap();
    let coords = (new_target.x as i32, new_target.y as i32);
    if chunk.get(&coords).unwrap().traversible {
	target.0 = new_target;
    }   
}

fn setup_camera(mut commands: Commands) {
    let mut camera = Camera2dBundle::default();
    camera.projection.scaling_mode = bevy::render::camera::ScalingMode::Auto {
	min_width: 16.0,
	min_height: 8.0,
    };
    commands.spawn(camera);
}

fn setup_player(mut commands: Commands, assets: Res<AssetServer>) {
    commands.spawn((
	player::PlayerBundle {
	    speed: player::Speed(2.5), // Tiles per second
	    target: player::Target(Vec2::splat(1.0)),
	},
	Controller,
	SpriteBundle {
	    texture: assets.load("player.png"),
	    transform: Transform::from_translation(Vec3::splat(1.0)),
	    sprite: Sprite {
		anchor: bevy::sprite::Anchor::BottomLeft,
		custom_size: Some(Vec2::new(1.0, 2.0)),
		..default()
	    },
	    ..default()
	},
    ));
}
    
pub struct PlayerPlugin;

impl bevy::app::Plugin for PlayerPlugin {
    fn build(&self, app: &mut bevy::app::App) {
	app
	    .add_startup_system(setup_player)
	    .add_startup_system(setup_camera)
	    .add_system(handle_input);
    }
}
