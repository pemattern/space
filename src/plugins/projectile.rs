use bevy::prelude::*;

use crate::core::projectile::Projectile;

pub struct ProjectilePlugin;
impl Plugin for ProjectilePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_projectiles);
    }
}

fn update_projectiles(
    mut commands: Commands,
    time: Res<Time>,
    mut projectile_query: Query<(Entity, &mut Projectile)>,
) {
    for (projectile_entity, mut projectile) in projectile_query.iter_mut() {
        projectile.lifetime -= time.delta_seconds();
        if projectile.lifetime <= 0.0 {
            commands.entity(projectile_entity).despawn();
        }
    }
}
