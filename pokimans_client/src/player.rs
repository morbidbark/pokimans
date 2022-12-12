use bevy::prelude::*;
use pokimans_common::game::player;

const UP: Vec3 = Vec3::new(0.0, 1.0, 0.0);
const DOWN: Vec3 = Vec3::new(0.0, -1.0, 0.0);
const LEFT: Vec3 = Vec3::new(-1.0, 0.0, 0.0);
const RIGHT: Vec3 = Vec3::new(1.0, 0.0, 0.0);
const ZERO: Vec3 = Vec3::new(0.0, 0.0, 0.0);

#[derive(Component)]
pub struct Controller;
pub fn handle_input(
    time: Res<Time>,
    input: Res<Input<KeyCode>>,
    mut query: Query<(&mut player::Speed, &mut Transform, &mut player::Direction), With<Controller>>,
    mut camera: Query<&mut Transform, (With<Camera2d>, Without<Controller>)>,
) {
    let mut new_direction = ZERO;

    for key in input.get_pressed() {
	match key {
	    KeyCode::W => { new_direction += UP },
	    KeyCode::A => { new_direction += LEFT },
	    KeyCode::S => { new_direction += DOWN },
	    KeyCode::D => { new_direction += RIGHT },
	    _ => (),
	}
    }

    let (speed, mut transform, mut direction) = query.get_single_mut().unwrap();
    direction.0 = new_direction;
    transform.translation += direction.0 * time.delta_seconds() * speed.0;

    //move camera to player position
   camera.get_single_mut().unwrap().translation = transform.translation; 
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
    let player = commands.spawn((
	player::PlayerBundle {
	    speed: player::Speed(2.5), // Tiles per second
	    direction: player::Direction(ZERO),
	},
	SpatialBundle {
	    transform: Transform::from_translation(Vec3::new(2.0, 2.0, 1.0)),
	    ..default()
	},
	Controller
    )).id();

    let player_sprite = commands.spawn(SpriteBundle {
	texture: assets.load("player.png"),
	sprite: Sprite {
	    anchor: bevy::sprite::Anchor::BottomLeft,
	    custom_size: Some(Vec2::new(1.0, 2.0)),
	    ..default()
	},
	..default()
    }).id();

    commands.entity(player).add_child(player_sprite);
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
