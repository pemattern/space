mod core;
mod plugins;
mod systems;

use bevy::{core_pipeline::prepass::DepthPrepass, prelude::*, window::WindowResolution};
use bevy_rapier3d::prelude::*;
use core::player::Player;
use plugins::{
    chromatic_abberation::{ChromaticAbberationPlugin, ChromaticAbberationSettings},
    procedural_skybox::ProceduralSkyboxPlugin,
    volumetric_nebula::{VolumetricNebulaPlugin, VolumetricNebulaSettings},
};
use systems::{player_movement::player_movement, spawn_asteroids::spawn_asteroids};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: WindowResolution::new(640.0, 480.0),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugins(ProceduralSkyboxPlugin)
        .add_plugins(VolumetricNebulaPlugin)
        .add_plugins(ChromaticAbberationPlugin)
        .add_systems(Startup, setup_system)
        .add_systems(Startup, spawn_asteroids)
        .add_systems(Update, player_movement)
        .add_systems(Update, camera_follow_system)
        .run();
}

#[derive(Component)]
struct CameraFollow {
    target_offset: Vec3,
}

fn setup_system(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, 3.0, 14.0),
            ..default()
        },
        DepthPrepass,
        CameraFollow {
            target_offset: Vec3::new(0., 3., 14.),
        },
        VolumetricNebulaSettings {
            time: 0.0,
            camera_position: Vec3::ZERO,
            camera_right: Vec3::ZERO,
            camera_up: Vec3::ZERO,
            camera_forward: Vec3::ZERO,
            light_direction: Vec3::ZERO,
            speed: 2.0,
            scale: 0.1,
            iso_value: 0.86,
            step_count: 100,
            step_distance: 3.0,
        },
        ChromaticAbberationSettings {
            intensity: 0.01,
            distance_exponent: 4.0,
        },
    ));

    commands.spawn(DirectionalLightBundle { ..default() });

    commands.spawn((
        Player,
        PbrBundle {
            mesh: meshes.add(ConicalFrustum::default()),
            material: materials.add(StandardMaterial {
                base_color: Color::srgb(0.7, 0.15, 0.15),
                ..default()
            }),
            ..default()
        },
        RigidBody::Dynamic,
        Damping {
            linear_damping: 0.9,
            angular_damping: 0.9,
        },
        Collider::capsule_y(1.0, 1.0),
        Velocity::default(),
        ExternalForce::default(),
        GravityScale(0.0),
    ));
}

fn camera_follow_system(
    player_query: Query<&Transform, With<Player>>,
    mut camera_query: Query<(&mut Transform, &CameraFollow), Without<Player>>,
    time: Res<Time>,
) {
    if let Ok(player_transform) = player_query.get_single() {
        for (mut camera_transform, camera_follow) in &mut camera_query {
            let target_offset_relative_to_player = player_transform
                .rotation
                .mul_vec3(camera_follow.target_offset);
            let target_position = player_transform.translation + target_offset_relative_to_player;
            camera_transform.translation = camera_transform
                .translation
                .lerp(target_position, 20.0 * time.delta_seconds());
            camera_transform.rotation = camera_transform
                .rotation
                .slerp(player_transform.rotation, 20.0 * time.delta_seconds());
        }
    }
}
