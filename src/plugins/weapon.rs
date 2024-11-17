use std::sync::Arc;

use bevy::prelude::*;

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

fn update_weapon_cooldowns(time: Res<Time>, mut weapon_query: Query<&mut WeaponSlots>) {
    let delta_seconds = time.delta_seconds();
    for mut weapon_container in weapon_query.iter_mut() {
        for weapon_container_slot in weapon_container.slots.iter_mut() {
            if let Some(weapon_slot) = &mut weapon_container_slot.1 {
                weapon_slot.update_cooldown(delta_seconds);
            }
        }
    }
}

pub struct WeaponPlugin;
impl Plugin for WeaponPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, add_components_player)
            .add_systems(Update, update_weapon_cooldowns)
            .insert_resource(Weapons::default());
    }
}

struct WeaponSlot {
    weapon: Arc<dyn Weapon>,
    remaining_cooldown: f32,
}

impl WeaponSlot {
    fn new(weapon: Arc<dyn Weapon>) -> Self {
        Self {
            weapon,
            remaining_cooldown: 0.0,
        }
    }

    pub fn fire(&mut self) {
        info!("fired");
        self.reset_cooldown();
    }

    pub fn update_cooldown(&mut self, delta_time: f32) {
        self.remaining_cooldown = self.remaining_cooldown - delta_time;
    }

    fn reset_cooldown(&mut self) {
        self.remaining_cooldown = self.weapon.cooldown();
    }
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
            if weapon_slot.remaining_cooldown <= 0.0 {
                weapon_slot.fire();
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
