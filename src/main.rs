use explore2::config;
use explore2::game;
use explore2::gui;
use explore2::logger;
use specs::prelude::*;

fn main() {
    let cfg = config::AppConfig::new();
    logger::new(&cfg);
    log::trace!("Using config: {:?}", cfg);

    log::debug!("Setting up GUI ...");
    let game_gui = gui::new(&cfg.gui);

    let title = cfg.game.title.clone();
    let context = rltk::RltkBuilder::simple(game_gui.width, game_gui.height)
        .with_title(title)
        .with_fullscreen(cfg.gui.fullscreen)
        .build();
    let mut gs = game::state::State {
        ecs: specs::World::new(),
    };

    game::world::setup(cfg, &mut gs);

    log::debug!("Inserting GUI into component system ...");
    gs.ecs.insert(game_gui);

    log::info!("Starting game ...");
    rltk::main_loop(context, gs);
}
