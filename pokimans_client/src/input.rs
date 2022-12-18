use bevy::prelude::*;
use pokimans_common::{
    movement::MoveEvent,
    utils::Vecs,
};

#[derive(Component)]
pub struct InputHandler;

pub fn handle_input(
    input: Res<Input<KeyCode>>,
    input_handler: Query<Entity, With<InputHandler>>,
    mut writer: EventWriter<MoveEvent>,
) {
    
    let mut direction = Vecs::ZERO;
    if input.pressed(KeyCode::W) { direction += Vecs::UP }
    else if input.pressed(KeyCode::A) { direction += Vecs::LEFT }
    else if input.pressed(KeyCode::S) { direction += Vecs::DOWN }
    else if input.pressed(KeyCode::D) { direction += Vecs::RIGHT }

    match input_handler.get_single() {
	Ok(entity) => writer.send(MoveEvent { entity, direction }),
	Err(e) => eprintln!("{}", e),
    }

    // Determine if the target location is traversible
//    let new_target = transform.translation.round().truncate() + direction;
//    let chunk = map.chunks.get(0).unwrap();
//    let coords = (new_target.x as i32, new_target.y as i32);
//    if chunk.get(&coords).unwrap().traversible {
//	target.0 = new_target;
//	network.tx.blocking_send(protocol::ClientMessage::PlayerMove { target: coords }).unwrap();
//    }   
}
