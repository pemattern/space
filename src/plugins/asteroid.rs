use bevy::prelude::*;
use bevy_rapier3d::prelude::{Collider, Damping, ExternalImpulse, RigidBody, Velocity};
use rand::Rng;

pub struct AsteroidPlugin;
impl Plugin for AsteroidPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_asteroids);
    }
}

#[derive(Component)]
pub struct Asteroid;

pub fn spawn_asteroids(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mut asteroid_mesh_handles: Vec<Handle<Mesh>> = Vec::new();
    let mesh_variants = 3;
    let asteroid_count = 1000;
    for variant in 0..mesh_variants {
        asteroid_mesh_handles.push(
            asset_server.load(
                GltfAssetLabel::Primitive {
                    mesh: variant,
                    primitive: 0,
                }
                .from_asset("meshes/asteroid.glb"),
            ),
        );
    }
    let asteroid_material_handle = materials.add(StandardMaterial {
        base_color: Color::srgb(0.25, 0.2, 0.15),
        ..default()
    });
    for _ in 0..asteroid_count {
        commands.spawn((
            Asteroid,
            PbrBundle {
                mesh: asteroid_mesh_handles[rand::thread_rng().gen_range(0..mesh_variants)].clone(),
                material: asteroid_material_handle.clone(),
                transform: Transform {
                    translation: random_vec3_in_sphere(),
                    scale: random_vec3(0.75, 1.25),
                    ..default()
                },
                ..default()
            },
            RigidBody::Dynamic,
            Damping {
                linear_damping: 0.0,
                angular_damping: 0.0,
            },
            Collider::capsule_y(1.0, 1.0),
            Velocity {
                linvel: random_vec3(-0.1, 0.1),
                angvel: random_vec3(-0.1, 0.1),
            },
        ));
    }
}

fn random_vec3(min: f32, max: f32) -> Vec3 {
    let mut rng = rand::thread_rng();
    Vec3::new(
        rng.gen_range(min..max),
        rng.gen_range(min..max),
        rng.gen_range(min..max),
    )
}

fn random_vec3_in_sphere() -> Vec3 {
    let radius = 300.0;
    let mut rng = rand::thread_rng();
    let r = radius * rng.gen::<f32>().cbrt();

    let theta = rng.gen_range(0.0..2.0 * std::f32::consts::PI);
    let phi = rng.gen_range(0.0..std::f32::consts::PI);

    let x = r * theta.sin() * phi.sin();
    let y = r * theta.cos() * phi.sin();
    let z = r * phi.cos();

    Vec3::new(x, y, z)
}
