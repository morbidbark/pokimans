use bevy::prelude::*;
use pokimans_common::tokio::setup_async_runtime;
use pokimans_common::game::map;
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
        .add_startup_system_to_stage(StartupStage::PreStartup, map::setup_map)
        .add_startup_system(renderer::render_map)

        .run();
}
