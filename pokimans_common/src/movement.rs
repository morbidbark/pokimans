use bevy::prelude::*;

pub struct MoveEvent {
    pub entity: Entity,
    pub direction: Vec2,
}

#[derive(Component)]
pub struct Speed(pub f32);

#[derive(Component)]
pub struct Target(pub Vec2);

#[derive(Component)]
pub struct MovementController;
 
#[derive(Bundle)]
pub struct MovementControllerBundle {
    _controller: MovementController,

    speed: Speed,
    target: Target,

    #[bundle]
    spatial: SpatialBundle,
}

impl MovementControllerBundle {
    pub fn new() -> Self {
	Self {
	    _controller: MovementController,
	    speed: Speed(2.5),
	    target: Target(Vec2::splat(1.0)),
	    spatial: SpatialBundle {
		transform: Transform::from_translation(Vec3::splat(1.0)),
		..default()
	    },
	}
    }
}

pub fn move_entities(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &Speed, &Target), With<MovementController>>,
){
    for (mut transform, speed, target) in query.iter_mut() {
	let target_direction = target.0 - transform.translation.truncate();
	if target_direction.length() > 0.01 {
	    transform.translation += target_direction.extend(0.0).normalize() * time.delta_seconds() * speed.0;
	}
    }
}

pub fn update_targets(
    map: Res<crate::map::Map>,
    mut events: EventReader<MoveEvent>,
    mut query: Query<(&Transform, &mut Target), With<MovementController>>,
) {
    for MoveEvent { entity, direction } in events.iter() {
	match query.get_mut(*entity) {
	    Ok((transform, mut target)) => {
		let new_target = transform.translation.round().truncate() + *direction;
		let chunk = map.chunks.get(0).unwrap();
		let coords = (new_target.x as i32, new_target.y as i32);
		if chunk.get(&coords).unwrap().traversible {
		    target.0 = new_target;
		    //network.tx.blocking_send(protocol::ClientMessage::PlayerMove { target: coords }).unwrap();
		}   
	    },
	    Err(e) => eprintln!("{}", e),
	}
    }
}
