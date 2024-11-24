use bevy::prelude::*;

#[derive(Component)]
pub struct Health {
    pub value: Gauge,
}

#[derive(Component)]
pub struct Shield {
    pub value: Gauge,
}

#[derive(Component)]
pub struct XP {
    pub level: usize,
    pub total: f32,
}

impl XP {
    pub fn xp_to_level(&self) -> f32 {
        1.0002_f32.powf(self.total).floor()
    }
}

pub struct Gauge {
    pub current: f32,
    pub max: f32,
}

impl Gauge {
    pub fn new(max: f32) -> Self {
        Self { current: max, max }
    }

    pub fn normalized(&self) -> f32 {
        if self.current > self.max {
            info!("currect value is greater than max");
            return 1.0;
        }
        if self.max <= 0.0 {
            info!("max value is 0");
            return 0.0;
        }
        return self.current / self.max;
    }
}
