use explore1::config::AppConfig;
use log;
use rltk::{Console, GameState, Rltk, RGB};
use specs::prelude::*;

use twyg;

mod components;
pub use components::*;
mod map;
pub use map::*;
mod player;
pub use player::*;
mod rect;
pub use rect::Rect;

pub struct State {
    pub ecs: World,
}

impl State {
    fn run_systems(&mut self) {
        self.ecs.maintain();
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut Rltk) {
        ctx.cls();

        self.run_systems();
        player_input(self, ctx);

        let map = self.ecs.fetch::<Vec<TileType>>();
        draw_map(&map, ctx);

        let positions = self.ecs.read_storage::<Position>();
        let renderables = self.ecs.read_storage::<Renderable>();

        for (pos, render) in (&positions, &renderables).join() {
            ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph);
        }
    }
}

fn main() {
    let cfg = AppConfig::new().unwrap();
    match twyg::setup_logger(&cfg.logging) {
        Ok(_) => {}
        Err(error) => panic!("Could not setup logger: {:?}", error),
    };
    log::info!("Successfully setup logger.");
    log::debug!("{:?}", cfg);

    use rltk::RltkBuilder;
    let context = RltkBuilder::simple80x50()
        .with_title(cfg.game.title)
        .build();
    let mut gs = State { ecs: World::new() };
    log::debug!("Registering components ...");
    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();
    gs.ecs.register::<Player>();

    log::debug!("Setting up Map ...");
    let (rooms, map) = new_map_rooms_and_corridors();
    gs.ecs.insert(map);
    let (player_x, player_y) = rooms[0].center();

    log::debug!("Setting up Player ...");
    gs.ecs
        .create_entity()
        .with(Position {
            x: player_x,
            y: player_y,
        })
        .with(Renderable {
            glyph: rltk::to_cp437(cfg.player.chr),
            fg: RGB::named(cfg.player.fg_color),
            bg: RGB::named(cfg.player.bg_color),
        })
        .with(Player {})
        .build();

    log::info!("Successfully compelted setup");
    log::info!("Starting game ...");
    rltk::main_loop(context, gs);
}
