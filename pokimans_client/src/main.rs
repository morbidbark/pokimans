use bevy::prelude::*;
use pokimans_common::tokio::setup_async_runtime;
use pokimans_client::net::setup_client;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system_to_stage(StartupStage::PreStartup, setup_async_runtime)
        .add_startup_system(setup_client)
        .run();
}

