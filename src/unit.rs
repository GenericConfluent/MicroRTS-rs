use bevy::prelude::*;
use serde::*;

use crate::agent::UnitAction;

#[derive(Hash, PartialEq, Eq, Serialize, Deserialize, Debug, Copy, Clone)]
pub enum UnitType {
    Worker,
    Barrack,
    Base,
    Ranged,
    Light,
    Heavy,
    Resource,
}

#[derive(Serialize, Deserialize)]
pub struct UnitInstance {
    pub utype: UnitType,
    pub health: u32,
    pub player: Option<u8>,
    pub position: UVec2,
}

impl UnitInstance {
    fn spawn(&self, commands: &mut Commands, config: &crate::config::Config) {
        use UnitType::*;
        let defaults = config.unit_defaults.get(&self.utype).unwrap();

        // FIXME: Health should be able to be overriden from the instance
        // struct definition.
        let mut entity = commands.spawn((
            UType(self.utype),
            Position(self.position),
            Player(self.player.unwrap()),
            Health(defaults.health),
        ));

        if matches!(self.utype, Worker | Ranged | Light | Heavy) {
            entity.insert(Attack(defaults.attack));
        }

        if self.utype == Worker {
            entity.insert(Resources(0));
        }
    }
}

#[derive(Component, Deref, DerefMut)]
pub struct Health(pub u32);

#[derive(Component, Deref, DerefMut)]
pub struct UType(pub UnitType);

#[derive(Component, Deref, DerefMut)]
pub struct Attack(pub u32);

// NOTE: This component is strictly for Worker units. Resource units remaining resources
// are tracked through the health component.
#[derive(Component, Deref, DerefMut)]
pub struct Resources(pub u32);

#[derive(Component, Deref, DerefMut)]
pub struct Position(pub UVec2);

#[derive(Component, Deref, DerefMut)]
pub struct Player(u8);

#[derive(Component, Deref, DerefMut)]
pub struct NextAction(Option<UnitAction>);
