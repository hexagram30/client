use crate::components;
use crate::game;
use crate::map;
use log;
use rltk::{Point, Rltk, VirtualKeyCode};
use specs;
use specs::prelude::*;
use std::cmp::{max, min};

pub fn try_move(delta_x: i32, delta_y: i32, ecs: &mut specs::World) {
    let mut positions = ecs.write_storage::<components::Position>();
    let mut players = ecs.write_storage::<components::Player>();
    let mut viewsheds = ecs.write_storage::<components::Viewshed>();
    let game_map = ecs.fetch::<map::Map>();

    for (_player, pos, viewshed) in (&mut players, &mut positions, &mut viewsheds).join() {
        let destination_idx = game_map.xy_idx(pos.x + delta_x, pos.y + delta_y);
        if game_map.tiles[destination_idx] != map::TileType::Wall {
            pos.x = min(game_map.width - 1, max(0, pos.x + delta_x));
            pos.y = min(game_map.height - 1, max(0, pos.y + delta_y));

            viewshed.dirty = true;
            let mut ppos = ecs.write_resource::<Point>();
            ppos.x = pos.x;
            ppos.y = pos.y;
        }
    }
}

pub fn input(gs: &mut game::State, ctx: &mut Rltk) -> game::RunState {
    // Player movement
    match ctx.key {
        None => return game::RunState::Paused, // Nothing happened
        Some(key) => match key {
            VirtualKeyCode::Left | VirtualKeyCode::A | VirtualKeyCode::Key4 => {
                try_move(-1, 0, &mut gs.ecs)
            }
            VirtualKeyCode::Right | VirtualKeyCode::D | VirtualKeyCode::Key6 => {
                try_move(1, 0, &mut gs.ecs)
            }
            VirtualKeyCode::Up | VirtualKeyCode::W | VirtualKeyCode::Key8 => {
                try_move(0, -1, &mut gs.ecs)
            }
            VirtualKeyCode::Down | VirtualKeyCode::S | VirtualKeyCode::Key2 => {
                try_move(0, 1, &mut gs.ecs)
            }
            VirtualKeyCode::Key7 | VirtualKeyCode::Q => try_move(-1, -1, &mut gs.ecs),
            VirtualKeyCode::Key9 | VirtualKeyCode::E => try_move(1, -1, &mut gs.ecs),
            VirtualKeyCode::Key3 | VirtualKeyCode::C => try_move(1, 1, &mut gs.ecs),
            VirtualKeyCode::Key1 | VirtualKeyCode::Z => try_move(-1, 1, &mut gs.ecs),
            VirtualKeyCode::Space => {
                log::debug!("Pausing game ...");
                return game::RunState::Paused;
            }
            VirtualKeyCode::Escape => {
                log::info!("Quitting ...");
                return game::RunState::Quitting;
            }
            _ => {
                log::debug!("Got user input: {:?}", key);
                return game::RunState::Paused;
            }
        },
    }
    game::RunState::Running
}
