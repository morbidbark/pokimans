use bevy::prelude::*;
use pokimans_common::tokio::setup_async_runtime;
use pokimans_server::net::setup_server;

fn main() {
    App::new()
        .add_plugins(MinimalPlugins)
        .add_startup_system_to_stage(StartupStage::PreStartup, setup_async_runtime)
        .add_startup_system(setup_server)
        .run();
}

