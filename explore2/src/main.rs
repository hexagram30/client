use explore2::components;
use explore2::config;
use explore2::game;
use explore2::logger;
use explore2::map;
use log;
use rltk::{self, Point, RGB};
use specs;
use specs::prelude::*;

fn main() {
    let cfg = config::AppConfig::new().unwrap();
    logger::new(&cfg);
    log::debug!("{:?}", cfg);

    let context = rltk::RltkBuilder::simple80x50()
        .with_title(cfg.game.title)
        .build();
    let mut gs = game::State {
        ecs: specs::World::new(),
        runstate: game::RunState::Running,
    };
    log::debug!("Registering components ...");
    gs.ecs.register::<components::Position>();
    gs.ecs.register::<components::Renderable>();
    gs.ecs.register::<components::Player>();
    gs.ecs.register::<components::Viewshed>();
    gs.ecs.register::<components::Monster>();
    gs.ecs.register::<components::Name>();

    log::debug!("Setting up Map ...");
    let game_map = map::Map::new_map_rooms_and_corridors();
    log::debug!("Created {} rooms.", game_map.rooms.len());
    let (player_x, player_y) = game_map.rooms[0].center();

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
            range: cfg.player.view_range.tile_count,
            dirty: true,
        })
        .with(components::Name {
            name: cfg.player.name,
        })
        .build();

    log::debug!("Setting up Monsters ...");
    let mut rng = rltk::RandomNumberGenerator::new();
    for (i, room) in game_map.rooms.iter().skip(1).enumerate() {
        log::trace!("Setting up Monster {} ...", i);
        let (x, y) = room.center();
        let glyph: u8;
        let name: String;
        let roll = rng.roll_dice(1, 2);
        match roll {
            1 => {
                glyph = rltk::to_cp437(cfg.monsters.monster1.chr);
                name = cfg.monsters.monster1.name.clone();
            }
            _ => {
                glyph = rltk::to_cp437(cfg.monsters.monster2.chr);
                name = cfg.monsters.monster2.name.clone();
            }
        }

        gs.ecs
            .create_entity()
            .with(components::Position { x, y })
            .with(components::Renderable {
                glyph,
                fg: RGB::named(cfg.monsters.fg_color),
                bg: RGB::named(cfg.monsters.bg_color),
            })
            .with(components::Viewshed {
                visible_tiles: Vec::new(),
                range: cfg.monsters.view_range.tile_count,
                dirty: true,
            })
            .with(components::Monster {})
            .with(components::Name {
                name: format!("{} #{}", &name, i),
            })
            .build();
    }

    log::info!("Successfully compelted setup");

    log::debug!("Inserting map and player into component system ...");
    gs.ecs.insert(game_map);
    gs.ecs.insert(Point::new(player_x, player_y));

    log::info!("Starting game ...");
    rltk::main_loop(context, gs);
}
