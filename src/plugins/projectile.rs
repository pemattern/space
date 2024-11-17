use bevy::prelude::*;

pub struct ProjectilePlugin;
impl Plugin for ProjectilePlugin {
    fn build(&self, app: &mut App) {}
}

#[derive(Component)]
pub struct ProjectileSpawner {
    pub relative_position: Vec3,
}

impl ProjectileSpawner {}

#[derive(Component)]
pub struct Projectile {
    pub speed: f32,
    pub lifetime: f32,
}
