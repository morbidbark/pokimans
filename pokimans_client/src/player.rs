use bevy::prelude::*;

const UP: Vec2 = Vec2 { x: 0.0, y: 1.0 };
const DOWN: Vec2 = Vec2 { x: 0.0, y: -1.0 };
const LEFT: Vec2 = Vec2 { x: -1.0, y: 0.0 };
const RIGHT: Vec2 = Vec2 { x: 1.0, y: 0.0 };
const ZERO: Vec2 = Vec2 { x: 0.0, y: 0.0 };
const SPEED: f32 = 5.0;

#[derive(Component)]
pub struct Controller;
pub fn capture_controller_input(time: Res<Time>, input: Res<Input<KeyCode>>, mut query: Query<&mut Transform, With<Controller>>, mut camera: Query<&mut Transform, (With<Camera2d>, Without<Controller>)>) {
    let mut direction = ZERO;

    for key in input.get_pressed() {
	match key {
	    KeyCode::W => { direction += UP },
	    KeyCode::A => { direction += LEFT },
	    KeyCode::S => { direction += DOWN },
	    KeyCode::D => { direction += RIGHT },
	    _ => (),
	}
    }
    
    query.get_single_mut().unwrap().translation += Vec3::new(direction.x, direction.y, 0.0) * time.delta_seconds() * SPEED;

    //move camera to player position
   camera.get_single_mut().unwrap().translation = query.get_single().unwrap().translation; 
}

pub fn setup_player(mut commands: Commands, assets: Res<AssetServer>) {
    let mut camera = Camera2dBundle::default();
    camera.projection.scaling_mode = bevy::render::camera::ScalingMode::Auto {
	min_width: 16.0,
	min_height: 8.0,
    };
    commands.spawn(camera);
    
    let texture = assets.load("player.png");
    commands.spawn((SpriteBundle {
	texture,
	transform: Transform::from_translation(Vec3::new(2.0,2.0,1.0)),
	sprite: Sprite {
	    anchor: bevy::sprite::Anchor::BottomLeft,
	    custom_size: Some(Vec2::splat(1.0)),
	    ..default()

	},
	..default()
    }, Controller));
}
    
