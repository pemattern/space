use bevy::{
    core_pipeline::prepass::{DepthPrepass, NormalPrepass},
    prelude::*,
};

use crate::core::{main_camera::MainCamera, player::Player};

pub struct MainCameraPlugin;

const TARGET_OFFSET: Vec3 = Vec3::new(0.0, 4.2, 14.0);

impl Plugin for MainCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, spawn_main_camera)
            .add_systems(Startup, add_components_main_camera)
            .add_systems(Update, camera_follow);
    }
}

fn spawn_main_camera(mut commands: Commands) {
    commands.spawn(MainCamera);
}

fn add_components_main_camera(
    mut commands: Commands,
    main_camera_query: Query<Entity, With<MainCamera>>,
) {
    if let Ok(main_camera_entity) = main_camera_query.get_single() {
        commands.entity(main_camera_entity).insert((
            Camera3dBundle {
                transform: Transform::from_translation(TARGET_OFFSET),
                ..default()
            },
            DepthPrepass,
            NormalPrepass,
        ));
    }
}

fn camera_follow(
    player_query: Query<&Transform, With<Player>>,
    mut camera_query: Query<&mut Transform, (With<MainCamera>, Without<Player>)>,
    time: Res<Time>,
) {
    if let Ok(player_transform) = player_query.get_single() {
        for mut camera_transform in &mut camera_query {
            let target_offset_relative_to_player =
                player_transform.rotation.mul_vec3(TARGET_OFFSET);
            let target_position = player_transform.translation + target_offset_relative_to_player;
            camera_transform.translation = camera_transform
                .translation
                .lerp(target_position, 10.0 * time.delta_seconds());
            camera_transform.rotation = camera_transform
                .rotation
                .slerp(player_transform.rotation, 10.0 * time.delta_seconds());
        }
    }
}
