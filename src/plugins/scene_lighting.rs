use bevy::prelude::*;
pub struct SceneLightingPlugin;

fn spawn_lights(mut commands: Commands) {
    commands.spawn(DirectionalLightBundle { ..default() });
}

impl Plugin for SceneLightingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_lights);
    }
}
