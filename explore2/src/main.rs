use explore2::config::AppConfig;
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
mod visibility_system;
use visibility_system::VisibilitySystem;

pub struct State {
    pub ecs: World,
}

impl State {
    fn run_systems(&mut self) {
        let mut vis = VisibilitySystem {};
        vis.run_now(&self.ecs);
        self.ecs.maintain();
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut Rltk) {
        ctx.cls();

        player_input(self, ctx);
        self.run_systems();

        draw_map(&self.ecs, ctx);

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
    gs.ecs.register::<Viewshed>();

    log::debug!("Setting up Map ...");
    let map: Map = Map::new_map_rooms_and_corridors();
    let (player_x, player_y) = map.rooms[0].center();
    gs.ecs.insert(map);

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
        .with(Viewshed {
            visible_tiles: Vec::new(),
            range: cfg.view_range.tile_count,
            dirty: true,
        })
        .build();

    log::info!("Successfully compelted setup");
    log::info!("Starting game ...");
    rltk::main_loop(context, gs);
}
