use crate::components;
use crate::config;
use crate::game;
use crate::map;
use crate::player;
use log;
use specs;
use specs::error::NoError;
use specs::prelude::*;
use specs::saveload::{SimpleMarker, SimpleMarkerAllocator, SerializeComponents, DeserializeComponents, MarkedBuilder};
use std::fs;

pub fn setup(cfg: config::AppConfig, gs: &mut game::state::State) {
    log::info!("Starting world setup ...");
    let title = cfg.game.title.clone();
    log::debug!("Setting up game log ...");
    let game_log = game::log::GameLog {
        entries: vec![format!("{} {}", cfg.game.welcome.clone(), title)],
    };

    log::debug!("Registering components ...");
    gs.ecs.register::<components::Position>();
    gs.ecs.register::<components::Renderable>();
    gs.ecs.register::<components::Player>();
    gs.ecs.register::<components::Viewshed>();
    gs.ecs.register::<components::Monster>();
    gs.ecs.register::<components::Name>();
    gs.ecs.register::<components::BlocksTile>();
    gs.ecs.register::<components::CombatStats>();
    gs.ecs.register::<components::WantsToMelee>();
    gs.ecs.register::<components::SufferDamage>();
    gs.ecs.register::<components::Item>();
    gs.ecs.register::<components::ProvidesHealing>();
    gs.ecs.register::<components::InflictsDamage>();
    gs.ecs.register::<components::AreaOfEffect>();
    gs.ecs.register::<components::Consumable>();
    gs.ecs.register::<components::Ranged>();
    gs.ecs.register::<components::InBackpack>();
    gs.ecs.register::<components::WantsToPickupItem>();
    gs.ecs.register::<components::WantsToUseItem>();
    gs.ecs.register::<components::WantsToDropItem>();
    gs.ecs.register::<components::Confusion>();
    gs.ecs.register::<SimpleMarker<components::SerializeMe>>();
    gs.ecs.register::<components::SerializationHelper>();

    log::debug!("Inserting serializer helper ...");
    gs.ecs.insert(SimpleMarkerAllocator::<components::SerializeMe>::new());
    log::debug!("Inserting RNG ...");
    gs.ecs.insert(rltk::RandomNumberGenerator::new());

    log::info!("Starting map setup ...");
    let game_map = map::new(&cfg, gs);
    let character = player::character::new(&cfg, gs, &game_map);
    log::info!("Completed map setup");

    log::debug!("Inserting configuration into component system ...");
    gs.ecs.insert(cfg.game);
    log::debug!("Inserting game log into component system ...");
    gs.ecs.insert(game_log);
    log::debug!("Inserting map into component system ...");
    gs.ecs.insert(game_map);
    log::debug!("Inserting player into component system ...");
    gs.ecs.insert(character.location);
    gs.ecs.insert(character.entity);
    gs.ecs.insert(game::state::RunState::PreRun);
    log::info!("Completed world setup");
}

pub fn delete (ecs: &mut World) {
    // Delete everything
    let mut to_delete = Vec::new();
    for e in ecs.entities().join() {
        to_delete.push(e);
    }
    for del in to_delete.iter() {
        ecs.delete_entity(*del).expect("Deletion failed");
    }
}

pub fn update (ecs: &mut World) {
    let mut deleteme: Option<Entity> = None;
    {
        let entities = ecs.entities();
        let helper = ecs.read_storage::<components::SerializationHelper>();
        let player = ecs.read_storage::<components::Player>();
        let position = ecs.read_storage::<components::Position>();
        let game_map = ecs.fetch::<map::Map>();
        for (e,h) in (&entities, &helper).join() {
            let mut worldmap = ecs.write_resource::<map::Map>();
            *worldmap = h.map.clone();
            worldmap.tile_content = vec![Vec::new(); game_map.width as usize * game_map.height as usize];
            deleteme = Some(e);
        }
        for (e,_p,pos) in (&entities, &player, &position).join() {
            let mut ppos = ecs.write_resource::<rltk::Point>();
            *ppos = rltk::Point::new(pos.x, pos.y);
            let mut player_resource = ecs.write_resource::<Entity>();
            *player_resource = e;
        }
    }
    ecs.delete_entity(deleteme.unwrap()).expect("Unable to delete helper");
}

macro_rules! serialize_individually {
    ($ecs:expr, $ser:expr, $data:expr, $( $type:ty),*) => {
        $(
        SerializeComponents::<NoError, SimpleMarker<components::SerializeMe>>::serialize(
            &( $ecs.read_storage::<$type>(), ),
            &$data.0,
            &$data.1,
            &mut $ser,
        )
        .unwrap();
        )*
    };
}

pub fn unload(ecs: &mut World, writer: fs::File) {
    let data = ( ecs.entities(), ecs.read_storage::<SimpleMarker<components::SerializeMe>>() );
    let mut serializer = serde_json::Serializer::new(writer);
    serialize_individually!(ecs, serializer, data, components::Position, components::Renderable, components::Player, components::Viewshed, components::Monster,
        components::Name, components::BlocksTile, components::CombatStats, components::SufferDamage, components::WantsToMelee, components::Item, components::Consumable, components::Ranged, components::InflictsDamage,
        components::AreaOfEffect, components::Confusion, components::ProvidesHealing, components::InBackpack, components::WantsToPickupItem, components::WantsToUseItem,
        components::WantsToDropItem, components::SerializationHelper
    );
}

macro_rules! deserialize_individually {
    ($ecs:expr, $de:expr, $data:expr, $( $type:ty),*) => {
        $(
        DeserializeComponents::<NoError, _>::deserialize(
            &mut ( &mut $ecs.write_storage::<$type>(), ),
            &mut $data.0, // entities
            &mut $data.1, // marker
            &mut $data.2, // allocater
            &mut $de,
        )
        .unwrap();
        )*
    };
}

pub fn load(ecs: &mut World, data: String) {
    
    let mut de = serde_json::Deserializer::from_str(&data);
    // XXX This is currently broken ...
    let mut d = (&mut ecs.entities_mut(), 
                    &mut ecs.write_storage::<SimpleMarker<components::SerializeMe>>(), 
                    &mut ecs.write_resource::<SimpleMarkerAllocator<components::SerializeMe>>());

    deserialize_individually!(ecs, de, d, components::Position, components::Renderable, components::Player, components::Viewshed, components::Monster,
        components::Name, components::BlocksTile, components::CombatStats, components::SufferDamage, components::WantsToMelee, components::Item, components::Consumable, components::Ranged, components::InflictsDamage,
        components::AreaOfEffect, components::Confusion, components::ProvidesHealing, components::InBackpack, components::WantsToPickupItem, components::WantsToUseItem,
        components::WantsToDropItem, components::SerializationHelper
    );
}
