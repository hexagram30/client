use super::{Map, Player, Position, State, TileType, Viewshed};
use log;
use rltk::{Rltk, VirtualKeyCode};
use specs::prelude::*;
use std::cmp::{max, min};

pub fn try_move_player(delta_x: i32, delta_y: i32, ecs: &mut World) {
    let mut positions = ecs.write_storage::<Position>();
    let mut players = ecs.write_storage::<Player>();
    let mut viewsheds = ecs.write_storage::<Viewshed>();
    let map = ecs.fetch::<Map>();

    for (_player, pos, viewshed) in (&mut players, &mut positions, &mut viewsheds).join() {
        let destination_idx = map.xy_idx(pos.x + delta_x, pos.y + delta_y);
        if map.tiles[destination_idx] != TileType::Wall {
            pos.x = min(79, max(0, pos.x + delta_x));
            pos.y = min(49, max(0, pos.y + delta_y));

            viewshed.dirty = true;
        }
    }
}

pub fn player_input(gs: &mut State, ctx: &mut Rltk) {
    // Player movement
    match ctx.key {
        None => {} // No player input
        Some(key) => match key {
            VirtualKeyCode::Left | VirtualKeyCode::A | VirtualKeyCode::Key4 => {
                try_move_player(-1, 0, &mut gs.ecs)
            }
            VirtualKeyCode::Right | VirtualKeyCode::D | VirtualKeyCode::Key6 => {
                try_move_player(1, 0, &mut gs.ecs)
            }
            VirtualKeyCode::Up | VirtualKeyCode::W | VirtualKeyCode::Key8 => {
                try_move_player(0, -1, &mut gs.ecs)
            }
            VirtualKeyCode::Down | VirtualKeyCode::S | VirtualKeyCode::Key2 => {
                try_move_player(0, 1, &mut gs.ecs)
            }
            VirtualKeyCode::Key7 => try_move_player(-1, -1, &mut gs.ecs),
            VirtualKeyCode::Key9 => try_move_player(1, -1, &mut gs.ecs),
            VirtualKeyCode::Key3 => try_move_player(1, 1, &mut gs.ecs),
            VirtualKeyCode::Key1 => try_move_player(-1, 1, &mut gs.ecs),
            _ => log::debug!("Got user input: {:?}", key),
        },
    }
}
