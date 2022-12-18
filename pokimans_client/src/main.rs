use bevy::prelude::*;
use pokimans_common::tokio::setup_async_runtime;
use pokimans_common::map::setup_map;
use pokimans_common::movement::{update_targets, move_entities};
use pokimans_client::net::setup_client;
use pokimans_client::renderer;

fn main() {
    App::new()
    // default_nearest prevents blurry pixel art
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugin(pokimans_client::player::PlayerPlugin)

        .add_startup_system_to_stage(StartupStage::PreStartup, setup_async_runtime)
        .add_startup_system_to_stage(StartupStage::PreStartup, renderer::setup_map_renderer)

        .add_startup_system(setup_client)
        .add_startup_system_to_stage(StartupStage::PreStartup, setup_map)
        .add_startup_system(renderer::render_map)

        .add_system(update_targets)
        .add_system(move_entities)

        .run();
}
