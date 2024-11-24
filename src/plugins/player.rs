use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

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
    if let Ok(player_entity) = player_query.get_single() {
        commands.entity(player_entity).insert((
            PbrBundle {
                mesh: asset_server.load(
                    GltfAssetLabel::Primitive {
                        mesh: 0,
                        primitive: 0,
                    }
                    .from_asset("meshes/spaceship.glb"),
                ),
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
            TransformInterpolation::default(),
        ));
    }
}

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, spawn_player)
            .add_systems(Startup, add_components_player);
    }
}
