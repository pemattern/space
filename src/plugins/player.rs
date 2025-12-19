use avian3d::prelude::{
    AngularDamping, AngularVelocity, Collider, LinearDamping, LinearVelocity, RigidBody,
    TransformInterpolation,
};
use bevy::prelude::*;

use crate::core::player::Player;

pub struct PlayerPlugin;

fn spawn_player(mut commands: Commands) {
    commands.spawn(Player);
}

fn add_components_player(
    mut commands: Commands,
    player_query: Query<Entity, With<Player>>,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    if let Ok(player_entity) = player_query.single() {
        commands.entity(player_entity).insert((
            Mesh3d(
                asset_server.load(
                    GltfAssetLabel::Primitive {
                        mesh: 0,
                        primitive: 0,
                    }
                    .from_asset("meshes/spaceship.glb"),
                ),
            ),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: Color::srgb(0.7, 0.15, 0.15),
                ..default()
            })),
            TransformInterpolation,
            LinearVelocity::ZERO,
            LinearDamping(0.9),
            AngularVelocity::ZERO,
            AngularDamping(0.9),
            RigidBody::Dynamic,
            Collider::sphere(0.5),
        ));
    }
}

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, spawn_player)
            .add_systems(Startup, add_components_player);
    }
}
