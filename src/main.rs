mod core;
mod plugins;

use bevy::{dev_tools::fps_overlay::FpsOverlayPlugin, prelude::*, window::WindowResolution};
use bevy_hanabi::HanabiPlugin;
use bevy_rapier3d::plugin::{NoUserData, RapierPhysicsPlugin};
use plugins::{
    asteroid::AsteroidPlugin, chromatic_abberation::ChromaticAbberationPlugin,
    main_camera::MainCameraPlugin, player::PlayerPlugin, player_controller::PlayerControllerPlugin,
    procedural_skybox::ProceduralSkyboxPlugin, upgrade::UpgradePlugin,
    volumetric_nebula::VolumetricNebulaPlugin,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: WindowResolution::new(640.0, 480.0),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(MainCameraPlugin)
        .add_plugins(PlayerPlugin)
        .add_plugins(FpsOverlayPlugin::default())
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugins(ProceduralSkyboxPlugin)
        .add_plugins(VolumetricNebulaPlugin)
        .add_plugins(ChromaticAbberationPlugin)
        .add_plugins(PlayerControllerPlugin)
        .add_plugins(HanabiPlugin)
        .add_plugins(AsteroidPlugin)
        .add_plugins(UpgradePlugin)
        .run();
}
