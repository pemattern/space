mod core;
mod plugins;
mod resources;

use bevy::{
    dev_tools::fps_overlay::FpsOverlayPlugin,
    prelude::*,
    window::{CursorGrabMode, PrimaryWindow, WindowResolution},
};
use bevy_rapier3d::plugin::{NoUserData, RapierConfiguration, RapierPhysicsPlugin, TimestepMode};
use plugins::{
    asteroid::AsteroidPlugin, chromatic_abberation::ChromaticAbberationPlugin,
    main_camera::MainCameraPlugin, player::PlayerPlugin, player_controller::PlayerControllerPlugin,
    procedural_skybox::ProceduralSkyboxPlugin, upgrade::UpgradePlugin,
    volumetric_nebula::VolumetricNebulaPlugin, weapon::WeaponPlugin,
};
use resources::upgrade::Upgrades;

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
        .add_plugins(AsteroidPlugin)
        .add_plugins(UpgradePlugin)
        .add_plugins(WeaponPlugin)
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
        .add_systems(Startup, cursor_hack)
        .run();
}

fn cursor_hack(mut windows: Query<&mut Window, With<PrimaryWindow>>) {
    for mut window in windows.iter_mut() {
        let half_width = window.resolution.width() / 2.0;
        let half_height = window.resolution.height() / 2.0;
        window.set_cursor_position(Some(Vec2::new(half_width, half_height)));
        window.cursor.grab_mode = CursorGrabMode::Locked;
        window.cursor.visible = false;
    }
}
