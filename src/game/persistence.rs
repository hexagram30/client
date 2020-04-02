use crate::components;
use crate::config;
use crate::game::world;
use crate::map;
use specs::prelude::*;
use specs::saveload::{MarkedBuilder, SimpleMarker};
use std::fs;
use std::fs::File;
use std::path::Path;

pub fn save(mut ecs: &mut World) {
    // Create helper
    let cfg = ecs.get_mut::<config::AppConfig>().unwrap().clone().game;
    let mapcopy = ecs.get_mut::<map::Map>().unwrap().clone();
    let savehelper = ecs
        .create_entity()
        .with(components::SerializationHelper { map: mapcopy })
        .marked::<SimpleMarker<components::SerializeMe>>()
        .build();

    // Actually serialize
    let writer = File::create(cfg.savegame_path()).unwrap();
    world::unload(&mut ecs, writer);

    // Clean up
    ecs.delete_entity(savehelper).expect("Crash on cleanup");
}

pub fn file_exists(ecs: &World) -> bool {
    let cfg = ecs.fetch::<config::AppConfig>().game.clone();
    Path::new(cfg.savegame_path()).exists()
}

pub fn load(mut ecs: &mut World) {
    world::delete(&mut ecs);
    let data =
        fs::read_to_string(ecs.fetch_mut::<config::AppConfig>().game.savegame_path()).unwrap();
    world::load(&mut ecs, data);
    world::update(&mut ecs);
}

pub fn delete(ecs: &World) {
    let cfg = ecs.fetch::<config::AppConfig>().game.clone();
    let savegame = cfg.savegame_path();
    if Path::new(savegame).exists() {
        std::fs::remove_file(savegame).expect("Unable to delete file");
    }
}
