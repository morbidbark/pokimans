use bevy::prelude::*;
use pokimans_common::{
    map::setup_map,
    movement::{MoveEvent, move_entities},
    tokio::setup_async_runtime
};
use pokimans_server::{
    movement::update_targets,
    net::setup_server
};

fn main() {
    App::new()
        .add_plugins(MinimalPlugins)

        .add_event::<MoveEvent>()

	.add_startup_system_to_stage(StartupStage::PreStartup, setup_async_runtime)
        .add_startup_system_to_stage(StartupStage::PreStartup, setup_map)
        .add_startup_system(setup_server)

	.add_system(move_entities)
	.add_system(update_targets)
	
        .run();
}

