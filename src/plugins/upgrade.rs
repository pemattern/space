use std::{any::TypeId, sync::Arc};

use bevy::prelude::*;

use crate::{
    core::player::Player,
    resources::upgrade::{Upgrade, Upgrades},
};

pub struct UpgradePlugin;

impl Plugin for UpgradePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, add_components_player)
            .add_systems(PostStartup, add_upgrade)
            .add_systems(Update, update_upgrades);
    }
}

fn update_upgrades(container_query: Query<&UpgradeContainer, With<Player>>) {
    if let Ok(container) = container_query.get_single() {
        for upgrade in container.get_all() {
            upgrade.on_update();
        }
    }
}

fn add_components_player(mut commands: Commands, player_query: Query<Entity, With<Player>>) {
    if let Ok(player_entity) = player_query.get_single() {
        commands
            .entity(player_entity)
            .insert(UpgradeContainer::default());
    }
}

fn add_upgrade(
    upgrades: Res<Upgrades>,
    mut container_query: Query<&mut UpgradeContainer, With<Player>>,
) {
    if let Ok(mut container) = container_query.get_single_mut() {
        container.attach(upgrades.test.clone());
    }
}

#[derive(Component, Default)]
pub struct UpgradeContainer {
    upgrades: Vec<Arc<dyn Upgrade>>,
}

impl UpgradeContainer {
    pub fn get_all(&self) -> &[Arc<dyn Upgrade>] {
        &self.upgrades
    }

    pub fn attach(&mut self, upgrade: Arc<dyn Upgrade>) {
        self.upgrades.push(upgrade);
    }

    pub fn contains_upgrade_of_type<T: Upgrade>(&self) -> bool {
        let type_id = TypeId::of::<T>();
        self.upgrades
            .iter()
            .any(|upgrade| upgrade.type_id() == type_id)
    }
}
