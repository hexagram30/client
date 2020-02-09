use explore1::config::{AppConfig};
use log;
use rltk::{Console, GameState, Rltk, RGB, VirtualKeyCode};
use specs::prelude::*;
use specs_derive::*;
use std::cmp::{max, min};
use twyg;

#[derive(Component)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Component)]
struct Renderable {
    glyph: u8,
    fg: RGB,
    bg: RGB,
}

#[derive(Component, Debug)]
struct Player {}

fn try_move_player(delta_x: i32, delta_y: i32, ecs: &mut World) {
    let mut positions = ecs.write_storage::<Position>();
    let mut players = ecs.write_storage::<Player>();

    for (_player, pos) in (&mut players, &mut positions).join() {
        pos.x = min(79 , max(0, pos.x + delta_x));
        pos.y = min(49, max(0, pos.y + delta_y));
    }
}

fn player_input(gs: &mut State, ctx: &mut Rltk) {
    // Player movement
    match ctx.key {
        None => {} // No player input
        Some(key) => match key {
            VirtualKeyCode::Left | VirtualKeyCode::A | VirtualKeyCode::Key4 =>
                try_move_player(-1, 0, &mut gs.ecs),
            VirtualKeyCode::Right | VirtualKeyCode::D | VirtualKeyCode::Key6 => 
                try_move_player(1, 0, &mut gs.ecs),
            VirtualKeyCode::Up | VirtualKeyCode::W | VirtualKeyCode::Key8 => 
                try_move_player(0, -1, &mut gs.ecs),
            VirtualKeyCode::Down | VirtualKeyCode::S | VirtualKeyCode::Key2 => 
                try_move_player(0, 1, &mut gs.ecs),
            VirtualKeyCode::Key7 => try_move_player(-1, -1, &mut gs.ecs),
            VirtualKeyCode::Key9 => try_move_player(1, -1, &mut gs.ecs),
            VirtualKeyCode::Key3 => try_move_player(1, 1, &mut gs.ecs),
            VirtualKeyCode::Key1 => try_move_player(-1, 1, &mut gs.ecs),
            _ => log::debug!("Got user input: {:?}", key)
        },
    }
}

#[derive(Component)]
struct LeftMover {}

struct LeftWalker {}

impl<'a> System<'a> for LeftWalker {
    type SystemData = (ReadStorage<'a, LeftMover>, 
                        WriteStorage<'a, Position>);

    fn run(&mut self, (lefty, mut pos) : Self::SystemData) {
        for (_lefty,pos) in (&lefty, &mut pos).join() {
            pos.x -= 1;
            if pos.x < 0 { pos.x = 79; }
        }
    }
}

struct State {
    ecs: World
}

impl State {
    fn run_systems(&mut self) {
        let mut lw = LeftWalker{};
        lw.run_now(&self.ecs);
        self.ecs.maintain();
    }
}

impl GameState for State {
    fn tick(&mut self, ctx : &mut Rltk) {
        ctx.cls();
        
        self.run_systems();
        player_input(self, ctx);

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
        Ok(_) => {},
        Err(error) => {
            panic!("Could not setup logger: {:?}", error)
        },
    };
    log::info!("Successfully setup logger.");
    log::debug!("{:?}", cfg);
    
    use rltk::RltkBuilder;
    let context = RltkBuilder::simple80x50()
        .with_title(cfg.game.title)
        .build();
    let mut gs = State {
        ecs: World::new()
    };
    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();
    gs.ecs.register::<Player>();
    gs.ecs.register::<LeftMover>();

    log::debug!("Setting up Player ...");
    gs.ecs
        .create_entity()
        .with(Position { x: cfg.player.init_x, y: cfg.player.init_y })
        .with(Renderable {
            glyph: rltk::to_cp437(cfg.player.chr),
            fg: RGB::named(cfg.player.fg_color),
            bg: RGB::named(cfg.player.bg_color),
        })
        .with(Player{})
        .build();

    log::debug!("Setting up NPCs ...");
    for i in 0..cfg.npcs.count {
        gs.ecs
            .create_entity()
            .with(Position { x: i * cfg.npcs.init_x, y: cfg.npcs.init_y })
            .with(Renderable {
                glyph: rltk::to_cp437(cfg.npcs.chr),
                fg: RGB::named(cfg.npcs.fg_color),
                bg: RGB::named(cfg.npcs.bg_color),
            })
            .with(LeftMover{})
            .build();
    }

    log::info!("Successfully compelted setup");
    log::info!("Starting game ...");
    rltk::main_loop(context, gs);
}
