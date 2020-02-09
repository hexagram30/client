use log;
use rltk::{VirtualKeyCode, Rltk};
use specs::prelude::*;
use super::{Position, Player, TileType, xy_idx, State};
use std::cmp::{min, max};

pub fn try_move_player(delta_x: i32, delta_y: i32, ecs: &mut World) {
    let mut positions = ecs.write_storage::<Position>();
    let mut players = ecs.write_storage::<Player>();
    let map = ecs.fetch::<Vec<TileType>>();

    for (_player, pos) in (&mut players, &mut positions).join() {
        let destination_idx = xy_idx(pos.x + delta_x, pos.y + delta_y);
        if map[destination_idx] != TileType::Wall {
            pos.x = min(79 , max(0, pos.x + delta_x));
            pos.y = min(49, max(0, pos.y + delta_y));
        }
    }
}

pub fn player_input(gs: &mut State, ctx: &mut Rltk) {
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
