use std::{any::Any, sync::Arc};

use bevy::prelude::*;

#[derive(Default, Resource)]
pub struct Upgrades {
    pub test: Arc<TestUpgrade>,
}

pub trait Upgrade: Any + Send + Sync {
    fn on_attach(&self) {}
    fn on_update(&self) {}
    fn on_remove(&self) {}
}

#[derive(Default)]
pub struct TestUpgrade;
impl Upgrade for TestUpgrade {
    fn on_attach(&self) {
        info!("attached");
    }
    fn on_update(&self) {
        // info!("updated");
    }
    fn on_remove(&self) {
        info!("removed");
    }
}
