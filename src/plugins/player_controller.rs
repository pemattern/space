use avian3d::{
    math::{Vector, Vector3},
    prelude::LinearVelocity,
};
use bevy::{input::mouse::MouseMotion, prelude::*};

use crate::core::player::Player;

use super::weapon::{WeaponSlotType, WeaponSlots};

pub struct PlayerControllerPlugin;

impl Plugin for PlayerControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, add_player_controller)
            .add_systems(FixedUpdate, player_movement)
            .add_systems(Update, handle_player_input)
            .add_systems(Update, player_weapon_fire);
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
    if let Ok(player_entity) = player_query.single() {
        commands
            .entity(player_entity)
            .insert(PlayerController::new());
    }
}

fn handle_player_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut mouse_movement: MessageReader<MouseMotion>,
    mut player_query: Query<&mut PlayerController, With<Player>>,
) {
    if let Ok(mut player_controller) = player_query.single_mut() {
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
    mut player_query: Query<(&Transform, &PlayerController, &mut LinearVelocity), With<Player>>,
) {
    if let Ok((transform, player_controller, mut velocity)) = player_query.single_mut() {
        let movement_direction = transform.forward() * player_controller.movement_input.z;
        let mut rotation_direction = Vec3::ZERO;

        rotation_direction += transform.forward() * player_controller.rotation_input.z;
        rotation_direction += transform.right() * player_controller.rotation_input.y;
        rotation_direction += transform.up() * player_controller.rotation_input.x;
        rotation_direction = rotation_direction.normalize_or_zero();

        let movement_force = movement_direction * player_controller.movement_force_strength;
        let rotation_force = rotation_direction * movement_direction.length();

        velocity.x += movement_force.x;
        velocity.y += movement_force.y;
        velocity.z += movement_force.z;

        // velocity = velocity.clamp_length_max(player_controller.max_movement_speed);
    }
}

fn player_weapon_fire(
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    mut weapon_slots_query: Query<&mut WeaponSlots, With<Player>>,
) {
    if let Ok(mut weapon_slots) = weapon_slots_query.single_mut() {
        if mouse_button_input.pressed(MouseButton::Left) {
            weapon_slots.fire(&WeaponSlotType::Primary);
        }
        if mouse_button_input.pressed(MouseButton::Right) {
            weapon_slots.fire(&WeaponSlotType::Secondary);
        }
    }
}
