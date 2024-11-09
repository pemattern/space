use bevy::{input::mouse::MouseMotion, prelude::*};
use bevy_rapier3d::prelude::*;

use crate::core::player::Player;

pub fn player_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut mouse_movement: EventReader<MouseMotion>,
    mut player_query: Query<(&Transform, &mut ExternalForce, &mut Velocity), With<Player>>,
) {
    let mut max_movement_speed = 20.0;
    let max_rotation_speed = 5.0;
    let movement_force_strength = 500.0;
    if let Ok((transform, mut external_force, mut velocity)) = player_query.get_single_mut() {
        let mut movement_direction = Vec3::ZERO;
        let mut rotation_direction = Vec3::ZERO;

        if keyboard_input.any_pressed([KeyCode::ShiftLeft]) {
            max_movement_speed = 50.0;
        }
        if keyboard_input.any_pressed([KeyCode::ArrowLeft, KeyCode::KeyA]) {
            rotation_direction -= transform.forward() * 10.0;
        }
        if keyboard_input.any_pressed([KeyCode::ArrowRight, KeyCode::KeyD]) {
            rotation_direction += transform.forward() * 10.0;
        }
        if keyboard_input.any_pressed([KeyCode::ArrowUp, KeyCode::KeyW]) {
            movement_direction += transform.forward().as_vec3();
        }
        for event in mouse_movement.read() {
            let clamped_mouse_y = event.delta.y.clamp(-1.0, 1.0);
            rotation_direction -= transform.right() * clamped_mouse_y * 10.0;
        }

        let movement_force = movement_direction * movement_force_strength;
        let rotation_force = rotation_direction * movement_direction.length();

        external_force.force = movement_force;
        external_force.torque = rotation_force;

        velocity.linvel = velocity.linvel.clamp_length_max(max_movement_speed);
        velocity.angvel = velocity.angvel.clamp_length_max(max_rotation_speed);
    }
}
