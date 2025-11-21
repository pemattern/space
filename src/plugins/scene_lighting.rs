use bevy::prelude::*;
pub struct SceneLightingPlugin;

fn setup_lights(mut commands: Commands, mut ambient_light: ResMut<AmbientLight>) {
    commands.spawn(DirectionalLight {
        illuminance: 10_000.0,
        ..default()
    });
    ambient_light.brightness = 200.0;
}

impl Plugin for SceneLightingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_lights);
    }
}
