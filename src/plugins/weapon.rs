use crate::core::projectile::Projectile;
use std::sync::Arc;

use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::{
    core::player::Player,
    resources::weapons::{Weapon, Weapons},
};

fn add_components_player(
    mut commands: Commands,
    weapons: Res<Weapons>,
    player_query: Query<Entity, With<Player>>,
) {
    if let Ok(player_entity) = player_query.get_single() {
        commands.entity(player_entity).insert(WeaponSlots::new(
            Some(weapons.test.clone()),
            Some(weapons.test2.clone()),
        ));
    }
}

fn update_weapon_slots(
    mut commands: Commands,
    time: Res<Time>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut query: Query<(&mut WeaponSlots, &Transform)>,
) {
    for (mut weapon_slots, transform) in query.iter_mut() {
        for weapon_slot in weapon_slots.slots.iter_mut() {
            if let Some(weapon_slot) = &mut weapon_slot.1 {
                match weapon_slot.state {
                    WeaponSlotState::Cooldown(ref mut cooldown) => {
                        if *cooldown > 0.0 {
                            *cooldown -= time.delta_seconds();
                        }
                        if *cooldown <= 0.0 {
                            weapon_slot.state = WeaponSlotState::Ready;
                        }
                    }
                    WeaponSlotState::Fired => {
                        commands.spawn((
                            Projectile { lifetime: 1.0 },
                            RigidBody::Dynamic,
                            Collider::ball(0.1),
                            Velocity {
                                linvel: transform.forward() * 200.0,
                                angvel: Vec3::ZERO,
                            },
                            PbrBundle {
                                mesh: meshes.add(Capsule3d {
                                    radius: 0.1,
                                    half_length: 3.0,
                                }),
                                material: materials.add(StandardMaterial {
                                    base_color: Color::srgb(0.0, 1.0, 0.0),
                                    unlit: true,
                                    ..default()
                                }),
                                transform: Transform {
                                    translation: transform.translation + transform.forward() * 2.0,
                                    rotation: transform.rotation
                                        * Quat::from_rotation_x(std::f32::consts::FRAC_PI_2),
                                    ..default()
                                },
                                ..default()
                            },
                        ));
                        weapon_slot.state = WeaponSlotState::Cooldown(weapon_slot.weapon.cooldown())
                    }
                    _ => {}
                }
            }
        }
    }
}

pub struct WeaponPlugin;
impl Plugin for WeaponPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, add_components_player)
            .add_systems(Update, update_weapon_slots)
            .insert_resource(Weapons::default());
    }
}

struct WeaponSlot {
    weapon: Arc<dyn Weapon>,
    state: WeaponSlotState,
}

impl WeaponSlot {
    fn new(weapon: Arc<dyn Weapon>) -> Self {
        Self {
            weapon,
            state: WeaponSlotState::Ready,
        }
    }
}

pub enum WeaponSlotState {
    Ready,
    Cooldown(f32),
    Fired,
}

#[derive(Component)]
pub struct WeaponSlots {
    slots: [(WeaponSlotType, Option<WeaponSlot>); 2],
}

impl WeaponSlots {
    fn new(primary: Option<Arc<dyn Weapon>>, secondary: Option<Arc<dyn Weapon>>) -> Self {
        let primary_slot = match primary {
            Some(weapon) => Some(WeaponSlot::new(weapon)),
            None => None,
        };
        let secondary_slot = match secondary {
            Some(weapon) => Some(WeaponSlot::new(weapon)),
            None => None,
        };
        Self {
            slots: [
                (WeaponSlotType::Primary, primary_slot),
                (WeaponSlotType::Secondary, secondary_slot),
            ],
        }
    }

    fn replace(&mut self, weapon_slot_type: &WeaponSlotType, weapon: Arc<dyn Weapon>) {
        let slot = self.get_slot_mut(weapon_slot_type);
        *slot = Some(WeaponSlot::new(weapon));
    }

    pub fn fire(&mut self, weapon_slot_type: &WeaponSlotType) {
        if let Some(weapon_slot) = self.get_slot_mut(weapon_slot_type) {
            match weapon_slot.state {
                WeaponSlotState::Ready => {
                    weapon_slot.state = WeaponSlotState::Fired;
                }
                _ => {}
            }
        }
    }

    fn get_slot_mut(&mut self, weapon_slot_type: &WeaponSlotType) -> &mut Option<WeaponSlot> {
        let index = match weapon_slot_type {
            WeaponSlotType::Primary => 0,
            WeaponSlotType::Secondary => 1,
        };
        &mut self.slots[index].1
    }
}

#[derive(PartialEq)]
pub enum WeaponSlotType {
    Primary,
    Secondary,
}
