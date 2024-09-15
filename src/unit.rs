use bevy::prelude::*;
use serde::*;

#[derive(Hash, PartialEq, Eq, Serialize, Deserialize)]
pub enum UnitType {
    Worker,
    Barrack,
    Base,
    Ranged,
    Light,
    Heavy,
    Resource,
}

#[derive(Component, Deref, DerefMut)]
pub struct Health(pub u32);

#[derive(Component, Deref, DerefMut)]
pub struct UType(pub UnitType);

#[derive(Component, Deref, DerefMut)]
pub struct Attack(pub u32);

#[derive(Component, Deref, DerefMut)]
pub struct Resources(pub u32);

#[derive(Component, Deref, DerefMut)]
pub struct Position(pub UVec2);

#[derive(Component, Deref, DerefMut)]
pub struct Player(u8);
