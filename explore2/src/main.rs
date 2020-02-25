use explore2::config;
use explore2::components;
use explore2::game;
use explore2::logger;
use explore2::map;
use log;
use rltk::{self, RGB};
use specs;
use specs::prelude::*;

fn main() {
    let cfg = config::AppConfig::new().unwrap();
    logger::new(&cfg);
    log::debug!("{:?}", cfg);

    let context = rltk::RltkBuilder::simple80x50()
        .with_title(cfg.game.title)
        .build();
    let mut gs = game::State { ecs: specs::World::new() };
    log::debug!("Registering components ...");
    gs.ecs.register::<components::Position>();
    gs.ecs.register::<components::Renderable>();
    gs.ecs.register::<components::Player>();
    gs.ecs.register::<components::Viewshed>();

    log::debug!("Setting up Map ...");
    let game_map: map::Map = map::Map::new_map_rooms_and_corridors();
    let (player_x, player_y) = game_map.rooms[0].center();
    gs.ecs.insert(game_map);

    log::debug!("Setting up Player ...");
    gs.ecs
        .create_entity()
        .with(components::Position {
            x: player_x,
            y: player_y,
        })
        .with(components::Renderable {
            glyph: rltk::to_cp437(cfg.player.chr),
            fg: RGB::named(cfg.player.fg_color),
            bg: RGB::named(cfg.player.bg_color),
        })
        .with(components::Player {})
        .with(components::Viewshed {
            visible_tiles: Vec::new(),
            range: cfg.view_range.tile_count,
            dirty: true,
        })
        .build();

    log::info!("Successfully compelted setup");
    log::info!("Starting game ...");
    rltk::main_loop(context, gs);
}
