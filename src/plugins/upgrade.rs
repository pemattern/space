use bevy::prelude::*;

use crate::core::player::Player;

pub struct UpgradePlugin;

pub const TEST: TestUpgrade = TestUpgrade;

impl Plugin for UpgradePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, add_upgrade)
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

fn add_upgrade(mut container_query: Query<&mut UpgradeContainer, With<Player>>) {
    if let Ok(mut container) = container_query.get_single_mut() {
        // container.try_attach(UpgradeType::TestUpgrade);
    }
}

#[derive(Component, Default)]
pub struct UpgradeContainer {
    upgrades: Vec<Box<dyn Upgrade>>,
}

impl UpgradeContainer {
    pub fn get_all(&self) -> &Vec<Box<dyn Upgrade>> {
        &self.upgrades
    }

    pub fn try_attach(&mut self, upgrade_type: UpgradeType) -> bool {
        if self
            .upgrades
            .iter()
            .any(|upgrade| upgrade.get_type() == upgrade_type)
        {
            return false;
        }
        let upgrade = get_upgrade(upgrade_type);
        upgrade.on_attach();
        self.upgrades.push(upgrade);
        return true;
    }
}

pub trait Upgrade: Send + Sync {
    fn get_type(&self) -> UpgradeType;
    fn on_attach(&self) {}
    fn on_update(&self) {}
    fn on_remove(&self) {}
}

#[derive(Debug, PartialEq)]
pub enum UpgradeType {
    TestUpgrade,
}

pub struct TestUpgrade;
impl Upgrade for TestUpgrade {
    fn get_type(&self) -> UpgradeType {
        UpgradeType::TestUpgrade
    }

    fn on_attach(&self) {
        info!("attached");
    }
    fn on_update(&self) {
        info!("updated");
    }
    fn on_remove(&self) {
        info!("removed");
    }
}

pub fn get_upgrade(upgrade_type: UpgradeType) -> Box<dyn Upgrade> {
    match upgrade_type {
        UpgradeType::TestUpgrade => Box::new(TEST),
    }
}
