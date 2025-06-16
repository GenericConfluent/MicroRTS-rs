// How sad. If only BSN.
use std::collections::HashMap;

use crate::unit::*;
use bevy::math::UVec2;
use bevy::prelude::*;
use serde::*;

#[derive(Serialize, Deserialize)]
struct UnitDescriptor {
    pub health: u32,
    pub attack: u32,
    pub cost: u32,
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

#[derive(Clone, Serialize, Deserialize, Copy, Debug, PartialEq, Eq)]
pub enum AgentFormat {
    Json,
    Xml,
    Ron,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum AgentDescriptor {
    Program {
        program: String,
        args: Vec<String>,
        format: AgentFormat,
    },
    TcpSocket {
        format: AgentFormat,
    },
    RandomAI,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Player {
    pub resource: u32,
    pub agent_descriptor: AgentDescriptor,
}

#[derive(Deserialize, Serialize, Resource)]
pub struct Config {
    pub unit_defaults: HashMap<UnitType, UnitDescriptor>,
    pub map_size: UVec2,
    pub players: Vec<Player>,
    pub units: Vec<UnitInstance>,
}

impl Config {
    fn push_instance(&mut self, utype: UnitType, player: Option<u8>, position: UVec2) -> &mut Self {
        let descriptor = self
            .unit_defaults
            .get(&utype)
            .expect(&format!("Config missing descriptor for `{utype:?}`"));

        self.units.push(UnitInstance {
            utype,
            health: descriptor.health,
            player,
            position,
        });
        self
    }
}

/// I am going to define a standard game of MicroRTS
impl Default for Config {
    fn default() -> Self {
        let mut unit_defaults = HashMap::new();

        // FIXME: These values were not directly copied. I just assumed them for the sake of speed.
        unit_defaults.insert(
            UnitType::Worker,
            UnitDescriptor {
                cost: 1,
                attack: 1,
                health: 1,
            },
        );

        unit_defaults.insert(
            UnitType::Barrack,
            UnitDescriptor {
                cost: 5,
                attack: 0,
                health: 4,
            },
        );

        unit_defaults.insert(
            UnitType::Base,
            UnitDescriptor {
                cost: 10,
                attack: 0,
                health: 10,
            },
        );

        unit_defaults.insert(
            UnitType::Ranged,
            UnitDescriptor {
                cost: 2,
                attack: 2,
                health: 1,
            },
        );

        unit_defaults.insert(
            UnitType::Light,
            UnitDescriptor {
                cost: 2,
                attack: 2,
                health: 4,
            },
        );

        unit_defaults.insert(
            UnitType::Heavy,
            UnitDescriptor {
                cost: 2,
                attack: 3,
                health: 4,
            },
        );

        unit_defaults.insert(
            UnitType::Resource,
            UnitDescriptor {
                cost: u32::MAX,
                attack: 0,
                health: 20,
            },
        );

        let mut prototype = Self {
            unit_defaults,
            map_size: UVec2::new(8, 8),
            players: vec![
                Player {
                    resource: 4,
                    agent_descriptor: AgentDescriptor::Program {
                        program: "python3".to_string(),
                        args: vec!["agent.py".to_string()],
                        format: AgentFormat::Json,
                    },
                },
                Player {
                    resource: 4,
                    agent_descriptor: AgentDescriptor::RandomAI,
                },
            ],
            units: vec![],
        };

        // Now we need to layout the units
        prototype
            .push_instance(UnitType::Resource, None, UVec2::new(0, 0))
            .push_instance(UnitType::Resource, None, UVec2::new(7, 7))
            .push_instance(UnitType::Base, Some(0), UVec2::new(2, 1))
            .push_instance(UnitType::Base, Some(1), UVec2::new(5, 6))
            .push_instance(UnitType::Worker, Some(0), UVec2::new(1, 1))
            .push_instance(UnitType::Worker, Some(1), UVec2::new(6, 6));

        prototype
    }
}
