use bevy::prelude::*;
use pokimans_common::{
    map::Map,
    movement::{ MoveEvent, MovementController, Target },
    protocol::ClientMessage,
};
use crate::net::Network;


pub fn update_targets(
    map: Res<Map>,
    network: ResMut<Network>,
    mut events: EventReader<MoveEvent>,
    mut query: Query<(&Transform, &mut Target), With<MovementController>>,
) {
    for MoveEvent { entity, direction } in events.iter() {
	match query.get_mut(*entity) {
	    Ok((transform, mut target)) => {
		let new_target = transform.translation.round().truncate() + *direction;
		if target.0 == new_target { return; }
		let chunk = map.chunks.get(0).unwrap();
		let coords = (new_target.x as i32, new_target.y as i32);
		if chunk.get(&coords).unwrap().traversible {
		    target.0 = new_target;
		    network.tx.blocking_send(ClientMessage::PlayerMove { direction: *direction }).unwrap();
		}   
	    },
	    Err(e) => eprintln!("{}", e),
	}
    }
}
