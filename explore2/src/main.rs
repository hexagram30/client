use explore2::components;
use explore2::config;
use explore2::game;
use explore2::gui;
use explore2::logger;
use explore2::map;
use log;
use rltk::{self, Point, RGB};
use specs;
use specs::prelude::*;

fn main() {
    let cfg = config::AppConfig::new();
    let title = cfg.game.title.clone();
    logger::new(&cfg);
    log::debug!("{:?}", cfg);
    log::debug!("Setting up game log ...");
    let game_log = game::log::GameLog {
        entries: vec![format!("{} {}", cfg.game.welcome.clone(), title)],
    };
    log::debug!("Setting up GUI ...");
    let game_gui = gui::new(&cfg);

    let context = rltk::RltkBuilder::simple(game_gui.width, game_gui.height)
        .with_title(title)
        .with_fullscreen(cfg.map.fullscreen)
        .build();
    let mut gs = game::state::State {
        ecs: specs::World::new(),
    };
    log::debug!("Registering components ...");
    gs.ecs.register::<config::AppConfig>();
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

    log::debug!("Setting up Map ...");
    let game_map = map::Map::new_map_rooms_and_corridors(&cfg);
    log::debug!("Created {} rooms.", game_map.rooms.len());
    let (player_x, player_y) = game_map.rooms[0].center();

    log::debug!("Setting up Player ...");
    let player_entity = gs
        .ecs
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
            name: cfg.player.name.clone(),
        })
        .with(components::CombatStats {
            max_hp: cfg.player.stats.max_hp,
            hp: cfg.player.stats.starting_hp,
            defense: cfg.player.stats.defense,
            power: cfg.player.stats.power,
        })
        .build();

    log::debug!("Setting up Monsters ...");
    let mut rng = rltk::RandomNumberGenerator::new();
    for (i, room) in game_map.rooms.iter().skip(1).enumerate() {
        log::trace!("Setting up Monster {} ...", i);
        let (x, y) = room.center();
        let glyph: u8;
        let name: String;
        let stats: config::Stats;
        let roll = rng.roll_dice(1, 2);
        match roll {
            1 => {
                glyph = rltk::to_cp437(cfg.monsters.monster1.chr);
                name = cfg.monsters.monster1.name.clone();
                stats = cfg.monsters.monster1.stats.clone();
            }
            _ => {
                glyph = rltk::to_cp437(cfg.monsters.monster2.chr);
                name = cfg.monsters.monster2.name.clone();
                stats = cfg.monsters.monster2.stats.clone();
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
                range: cfg.monsters.view_range.tile_count.clone(),
                dirty: true,
            })
            .with(components::Monster {})
            .with(components::Name {
                name: format!("{} #{}", &name, i),
            })
            .with(components::BlocksTile {})
            .with(components::CombatStats {
                max_hp: stats.max_hp,
                hp: stats.starting_hp,
                defense: stats.defense,
                power: stats.power,
            })
            .build();
    }

    log::info!("Successfully compelted setup");

    log::debug!("Inserting configuration into component system ...");
    gs.ecs.insert(cfg);
    log::debug!("Inserting game log into component system ...");
    gs.ecs.insert(game_log);
    log::debug!("Inserting GUI into component system ...");
    gs.ecs.insert(game_gui);
    log::debug!("Inserting map into component system ...");
    gs.ecs.insert(game_map);
    log::debug!("Inserting player into component system ...");
    gs.ecs.insert(Point::new(player_x, player_y));
    gs.ecs.insert(player_entity);
    gs.ecs.insert(game::state::RunState::PreRun);

    log::info!("Starting game ...");
    rltk::main_loop(context, gs);
}
