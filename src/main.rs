mod core;
mod plugins;
mod resources;

use avian3d::{prelude::Gravity, PhysicsPlugins};
use bevy::{dev_tools::fps_overlay::FpsOverlayPlugin, prelude::*, window::WindowResolution};
use plugins::{
    asteroid::AsteroidPlugin, chromatic_abberation::ChromaticAbberationPlugin,
    main_camera::MainCameraPlugin, player::PlayerPlugin, player_controller::PlayerControllerPlugin,
    procedural_skybox::ProceduralSkyboxPlugin, projectile::ProjectilePlugin,
    scene_lighting::SceneLightingPlugin, upgrade::UpgradePlugin,
    volumetric_nebula::VolumetricNebulaPlugin, weapon::WeaponPlugin,
};
use resources::upgrades::Upgrades;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: WindowResolution::new(1920, 1080),
                // mode: bevy::window::WindowMode::BorderlessFullscreen(MonitorSelection::Primary),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(PhysicsPlugins::default())
        .add_plugins(MainCameraPlugin)
        .add_plugins(PlayerPlugin)
        .add_plugins(FpsOverlayPlugin::default())
        .add_plugins(ProceduralSkyboxPlugin)
        .add_plugins(VolumetricNebulaPlugin)
        .add_plugins(ChromaticAbberationPlugin)
        .add_plugins(PlayerControllerPlugin)
        .add_plugins(AsteroidPlugin)
        .add_plugins(UpgradePlugin)
        .add_plugins(WeaponPlugin)
        .add_plugins(ProjectilePlugin)
        .add_plugins(SceneLightingPlugin)
        .insert_resource(Upgrades::default())
        .insert_resource(Gravity::ZERO)
        .run();
}
