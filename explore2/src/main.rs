use explore2::components;
use explore2::config;
use explore2::game;
use explore2::gui;
use explore2::logger;
use explore2::map;
use explore2::monster;
use explore2::player;
use log;
use rltk::Point;
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
    log::debug!("Setting up Player ...");
    let (x, y) = game_map.rooms[0].center();
    let player_pos = components::Position { x: x, y: y };
    let player_entity = player::spawn(&mut gs.ecs, player_pos, cfg.player.clone());
    log::debug!("Inserting player into component system ...");
    gs.ecs.insert(Point::new(x, y));

    log::debug!("Inserting RNG ...");
    gs.ecs.insert(rltk::RandomNumberGenerator::new());

    log::debug!("Setting up Monsters ...");
    let mut rng = rltk::RandomNumberGenerator::new();
    for (i, room) in game_map.rooms.iter().skip(1).enumerate() {
        log::trace!("Setting up Monster {} ...", i);
        let (x, y) = room.center();
        let monster_pos = components::Position { x: x, y: y };
        monster::random(&mut gs.ecs, monster_pos, cfg.monsters.clone());
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

    gs.ecs.insert(player_entity);
    gs.ecs.insert(game::state::RunState::PreRun);

    log::info!("Starting game ...");
    rltk::main_loop(context, gs);
}
