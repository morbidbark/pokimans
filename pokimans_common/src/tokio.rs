use bevy::prelude::{Commands, Resource};

#[derive(Resource)]
pub struct Tokio {
    pub runtime: tokio::runtime::Runtime,
}

pub fn setup_async_runtime(mut commands: Commands) {
    commands.insert_resource(Tokio {
	runtime: tokio::runtime::Builder::new_multi_thread()
	    .worker_threads(4)
	    .enable_all()
	    .build()
	    .unwrap()
    });
}
