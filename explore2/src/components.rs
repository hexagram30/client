use crate::map;
use rltk::RGB;
use serde::{Deserialize, Serialize};
use specs::error::NoError;
use specs::prelude::*;
use specs::saveload::{ConvertSaveload, Marker};
use specs_derive::*;

#[derive(Clone, Component, ConvertSaveload, Debug)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

#[derive(Clone, Component, ConvertSaveload)]
pub struct Renderable {
    pub glyph: u8,
    pub fg: RGB,
    pub bg: RGB,
    pub render_order: i32,
}

#[derive(Clone, Component, Debug, Deserialize, Serialize)]
pub struct Player {}

#[derive(Clone, Component, ConvertSaveload)]
pub struct Viewshed {
    pub visible_tiles: Vec<rltk::Point>,
    pub range: i32,
    pub dirty: bool,
}

#[derive(Clone, Component, Debug, Deserialize, Serialize)]
pub struct Monster {}

#[derive(Clone, Component, ConvertSaveload, Debug)]
pub struct Name {
    pub name: String,
}

#[derive(Clone, Component, Debug, Deserialize, Serialize)]
pub struct BlocksTile {}

#[derive(Clone, Component, ConvertSaveload, Debug)]
pub struct CombatStats {
    pub max_hp: i32,
    pub hp: i32,
    pub defense: i32,
    pub power: i32,
}

#[derive(Clone, Component, ConvertSaveload, Debug)]
pub struct WantsToMelee {
    pub target: Entity,
}

#[derive(Clone, Component, ConvertSaveload, Debug)]
pub struct SufferDamage {
    pub amount: Vec<i32>,
}

#[derive(Clone, Component, Debug, Deserialize, Serialize)]
pub struct Item {}

#[derive(Clone, Component, Debug, Deserialize, Serialize)]
pub struct Consumable {}

#[derive(Clone, Component, ConvertSaveload, Debug)]
pub struct Ranged {
    pub range: i32,
}

#[derive(Clone, Component, ConvertSaveload, Debug)]
pub struct InflictsDamage {
    pub damage: i32,
}

#[derive(Clone, Component, ConvertSaveload, Debug)]
pub struct AreaOfEffect {
    pub radius: i32,
}

#[derive(Clone, Component, ConvertSaveload, Debug)]
pub struct Confusion {
    pub turns: i32,
}

#[derive(Clone, Component, ConvertSaveload, Debug)]
pub struct ProvidesHealing {
    pub heal_amount: i32,
}

#[derive(Clone, Component, ConvertSaveload, Debug)]
pub struct InBackpack {
    pub owner: Entity,
}

#[derive(Clone, Component, ConvertSaveload, Debug)]
pub struct WantsToPickupItem {
    pub collected_by: Entity,
    pub item: Entity,
}

#[derive(Clone, Component, ConvertSaveload, Debug)]
pub struct WantsToUseItem {
    pub item: Entity,
    pub target: Option<rltk::Point>,
}

#[derive(Clone, Component, ConvertSaveload, Debug)]
pub struct WantsToDropItem {
    pub item: Entity,
}

// Serialization helper code. We need to implement ConvertSaveload for each type that contains an
// Entity.

pub struct SerializeMe;

// Special component that exists to help serialize the game data
#[derive(Clone, Component, Deserialize, Serialize)]
pub struct SerializationHelper {
    pub map: map::Map,
}
