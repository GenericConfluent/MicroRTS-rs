// How sad. If only BSN.
use std::collections::HashMap;

use crate::unit::*;
use bevy::math::UVec2;
use bevy::prelude::*;
use serde::*;

#[derive(Serialize, Deserialize)]
struct UnitConfig {
    health: u32,
    attack: u32,
}

#[derive(Serialize, Deserialize)]
enum UnitComponent {
    Health(u32),
    UType(UnitType),
    Attack(u32),
    Resources(u32),
    Position(UVec2),
    Player(u8),
}

//
#[derive(Deserialize, Serialize, Resource)]
pub struct Config {
    unit_defaults: HashMap<UnitType, UnitConfig>,
    map_size: UVec2,
    default_resource_amount: u32,
    units: Vec<Vec<UnitComponent>>,
}

impl Default for Config {
    fn default() -> Self {
        let mut unit_defaults = HashMap::new();

        Self {
            unit_defaults,
            map_size: UVec2::new(8, 8),
            default_resource_amount: 20,
            units: vec![],
        }
    }
}
