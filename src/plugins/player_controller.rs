use bevy::{input::mouse::MouseMotion, prelude::*};
use bevy_rapier3d::prelude::*;

use crate::core::player::Player;

use super::weapon::{FiringMode, WeaponContainer};

pub struct PlayerControllerPlugin;

impl Plugin for PlayerControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, add_player_controller)
            .add_systems(FixedUpdate, player_movement)
            .add_systems(Update, handle_player_input);
    }
}

#[derive(Component, Default)]
struct PlayerController {
    pub movement_input: Vec3,
    pub rotation_input: Vec3,
    pub max_movement_speed: f32,
    pub max_rotation_speed: f32,
    pub movement_force_strength: f32,
}

impl PlayerController {
    pub fn new() -> Self {
        Self {
            max_movement_speed: 20.0,
            max_rotation_speed: 5.0,
            movement_force_strength: 500.0,
            ..default()
        }
    }
}

fn add_player_controller(mut commands: Commands, player_query: Query<Entity, With<Player>>) {
    if let Ok(player_entity) = player_query.get_single() {
        commands
            .entity(player_entity)
            .insert(PlayerController::new());
    }
}

fn handle_player_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut mouse_movement: EventReader<MouseMotion>,
    mut player_query: Query<&mut PlayerController, With<Player>>,
) {
    if let Ok(mut player_controller) = player_query.get_single_mut() {
        let mut movement_input = Vec3::ZERO;
        let mut rotation_input = Vec3::ZERO;
        if keyboard_input.any_pressed([KeyCode::KeyW]) {
            movement_input += Vec3::Z;
        }
        if keyboard_input.any_pressed([KeyCode::KeyA]) {
            rotation_input -= Vec3::Z;
        }
        if keyboard_input.any_pressed([KeyCode::KeyD]) {
            rotation_input += Vec3::Z;
        }
        for event in mouse_movement.read() {
            let clamped_mouse = event.delta.clamp(Vec2::NEG_ONE, Vec2::ONE);
            rotation_input -= Vec3::new(clamped_mouse.x, clamped_mouse.y, 0.0);
        }
        player_controller.movement_input = movement_input;
        player_controller.rotation_input = rotation_input;
    }
}

fn player_movement(
    mut player_query: Query<
        (
            &Transform,
            &PlayerController,
            &mut ExternalForce,
            &mut Velocity,
        ),
        With<Player>,
    >,
) {
    if let Ok((transform, player_controller, mut external_force, mut velocity)) =
        player_query.get_single_mut()
    {
        let movement_direction = transform.forward() * player_controller.movement_input.z;
        let mut rotation_direction = Vec3::ZERO;

        rotation_direction += transform.forward() * player_controller.rotation_input.z;
        rotation_direction += transform.right() * player_controller.rotation_input.y;
        rotation_direction += transform.up() * player_controller.rotation_input.x;
        rotation_direction = rotation_direction.normalize_or_zero();

        let movement_force = movement_direction * player_controller.movement_force_strength;
        let rotation_force = rotation_direction * movement_direction.length();

        external_force.force = movement_force;
        external_force.torque = rotation_force * 10.0;

        velocity.linvel = velocity
            .linvel
            .clamp_length_max(player_controller.max_movement_speed);
        velocity.angvel = velocity
            .angvel
            .clamp_length_max(player_controller.max_rotation_speed);
    }
}

fn player_weapon_fire(
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    mut weapon_container_query: Query<&mut WeaponContainer, With<Player>>,
) {
    if let Ok(mut weapon_container) = weapon_container_query.get_single_mut() {
        if mouse_button_input.pressed(MouseButton::Left) {
            weapon_container.fire(FiringMode::Primary);
        }
        if mouse_button_input.pressed(MouseButton::Right) {
            weapon_container.fire(FiringMode::Secondary);
        }
    }
}
