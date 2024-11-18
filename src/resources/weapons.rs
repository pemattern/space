use std::sync::Arc;

use bevy::prelude::*;

#[derive(Default, Resource)]
pub struct Weapons {
    pub test: Arc<TestWeapon>,
    pub test2: Arc<TestWeapon2>,
}

#[derive(Default)]
pub struct TestWeapon;
impl Weapon for TestWeapon {
    fn cooldown(&self) -> f32 {
        0.25
    }

    fn damage(&self) -> f32 {
        2.0
    }
}

#[derive(Default)]
pub struct TestWeapon2;
impl Weapon for TestWeapon2 {
    fn cooldown(&self) -> f32 {
        2.0
    }

    fn damage(&self) -> f32 {
        4.0
    }
}

pub trait Weapon: Send + Sync {
    fn cooldown(&self) -> f32;
    fn damage(&self) -> f32;
}
