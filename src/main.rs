mod core;
mod plugins;
mod resources;

use core::projectile::Projectile;

use bevy::{
    dev_tools::fps_overlay::FpsOverlayPlugin,
    pbr::wireframe::{WireframeConfig, WireframePlugin},
    prelude::*,
    window::WindowResolution,
};
use bevy_rapier3d::plugin::{NoUserData, RapierConfiguration, RapierPhysicsPlugin, TimestepMode};
use plugins::{
    asteroid::AsteroidPlugin, chromatic_abberation::ChromaticAbberationPlugin,
    main_camera::MainCameraPlugin, outline::OutlinePlugin, player::PlayerPlugin,
    player_controller::PlayerControllerPlugin, procedural_skybox::ProceduralSkyboxPlugin,
    projectile::ProjectilePlugin, scene_lighting::SceneLightingPlugin, upgrade::UpgradePlugin,
    volumetric_nebula::VolumetricNebulaPlugin, weapon::WeaponPlugin,
};
use resources::upgrades::Upgrades;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: WindowResolution::new(1920.0, 1080.0),
                // mode: bevy::window::WindowMode::BorderlessFullscreen,
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
        // .add_plugins(OutlinePlugin)
        .add_plugins(ChromaticAbberationPlugin)
        .add_plugins(PlayerControllerPlugin)
        .add_plugins(AsteroidPlugin)
        .add_plugins(UpgradePlugin)
        .add_plugins(WeaponPlugin)
        .add_plugins(ProjectilePlugin)
        .add_plugins(SceneLightingPlugin)
        .insert_resource(Upgrades::default())
        .insert_resource(RapierConfiguration {
            gravity: Vec3::ZERO,
            physics_pipeline_active: true,
            query_pipeline_active: true,
            timestep_mode: TimestepMode::Interpolated {
                dt: 1.0 / 60.0,
                time_scale: 1.0,
                substeps: 1,
            },
            scaled_shape_subdivision: 4,
            force_update_from_transform_changes: false,
        })
        .run();
}
