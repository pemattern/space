use bevy::prelude::*;

use crate::core::player::Player;

const TEST_WEAPON: TestWeapon = TestWeapon;
const TEST_WEAPON2: TestWeapon2 = TestWeapon2;

fn add_components_player(mut commands: Commands, player_query: Query<Entity, With<Player>>) {
    if let Ok(player_entity) = player_query.get_single() {
        commands
            .entity(player_entity)
            .insert(WeaponContainer::new());
    }
}

fn update_weapon_cooldowns(time: Res<Time>, mut weapon_query: Query<&mut WeaponContainer>) {
    let delta_seconds = time.delta_seconds();
    for mut weapon_container in weapon_query.iter_mut() {
        let primary = &mut weapon_container.primary;
        primary.remaining_cooldown = primary.remaining_cooldown - delta_seconds;
        let secondary = &mut weapon_container.secondary;
        secondary.remaining_cooldown = secondary.remaining_cooldown - delta_seconds;
    }
}

pub struct WeaponPlugin;
impl Plugin for WeaponPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, add_components_player)
            .add_systems(Update, update_weapon_cooldowns);
    }
}

struct WeaponSlot {
    weapon: Box<dyn Weapon>,
    remaining_cooldown: f32,
}

impl WeaponSlot {
    fn new(weapon: Box<dyn Weapon>) -> Self {
        Self {
            weapon,
            remaining_cooldown: 0.0,
        }
    }
}

#[derive(Component)]
pub struct WeaponContainer {
    primary: WeaponSlot,
    secondary: WeaponSlot,
}

impl WeaponContainer {
    pub fn new() -> Self {
        Self {
            primary: WeaponSlot::new(Box::new(TEST_WEAPON)),
            secondary: WeaponSlot::new(Box::new(TEST_WEAPON2)),
        }
    }

    pub fn fire(&mut self, firing_mode: FiringMode) {
        let weapon_slot = match firing_mode {
            FiringMode::Primary => &mut self.primary,
            FiringMode::Secondary => &mut self.secondary,
        };

        if weapon_slot.remaining_cooldown <= 0.0 {
            info!("fired: {} dmg", weapon_slot.weapon.damage());
            weapon_slot.remaining_cooldown = weapon_slot.weapon.cooldown();
        }
    }
}

pub enum FiringMode {
    Primary,
    Secondary,
}

pub struct TestWeapon;
impl Weapon for TestWeapon {
    fn cooldown(&self) -> f32 {
        1.0
    }

    fn damage(&self) -> f32 {
        2.0
    }
}

pub struct TestWeapon2;
impl Weapon for TestWeapon2 {
    fn cooldown(&self) -> f32 {
        2.0
    }

    fn damage(&self) -> f32 {
        4.0
    }
}

trait Weapon: Send + Sync {
    fn cooldown(&self) -> f32;
    fn damage(&self) -> f32;
}
