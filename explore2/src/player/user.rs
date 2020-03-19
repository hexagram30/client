use crate::components;
use crate::game;
use crate::map;
use crate::player::character;
use log;
use rltk::{Point, Rltk, VirtualKeyCode};
use specs;
use specs::prelude::*;
use std::cmp::{max, min};

pub fn try_move(delta_x: i32, delta_y: i32, ecs: &mut specs::World) {
    let mut positions = ecs.write_storage::<components::Position>();
    let players = ecs.read_storage::<components::Player>();
    let mut viewsheds = ecs.write_storage::<components::Viewshed>();
    let entities = ecs.entities();
    let combat_stats = ecs.read_storage::<components::CombatStats>();
    let game_map = ecs.fetch::<map::Map>();
    let mut wants_to_melee = ecs.write_storage::<components::WantsToMelee>();

    for (entity, _player, pos, viewshed) in
        (&entities, &players, &mut positions, &mut viewsheds).join()
    {
        if pos.x + delta_x < 1
            || pos.x + delta_x > game_map.width - 1
            || pos.y + delta_y < 1
            || pos.y + delta_y > game_map.height - 1
        {
            return;
        }
        let destination_idx = game_map.xy_idx(pos.x + delta_x, pos.y + delta_y);

        for potential_target in game_map.tile_content[destination_idx].iter() {
            let target = combat_stats.get(*potential_target);
            if let Some(_target) = target {
                wants_to_melee
                    .insert(
                        entity,
                        components::WantsToMelee {
                            target: *potential_target,
                        },
                    )
                    .expect("Add target failed");
                return;
            }
        }

        if !game_map.blocked[destination_idx] {
            pos.x = min(game_map.width - 1, max(0, pos.x + delta_x));
            pos.y = min(game_map.height - 1, max(0, pos.y + delta_y));

            viewshed.dirty = true;
            let mut ppos = ecs.write_resource::<Point>();
            ppos.x = pos.x;
            ppos.y = pos.y;
        }
    }
}

pub fn input(gs: &mut game::state::State, ctx: &mut Rltk) -> game::state::RunState {
    // Player movement
    match ctx.key {
        None => return game::state::RunState::AwaitingInput, // Nothing happened
        Some(key) => match key {
            // Movement
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
            // Items management
            VirtualKeyCode::P => character::get_item(&mut gs.ecs), // pick-up
            VirtualKeyCode::I => return game::state::RunState::ShowInventory,
            VirtualKeyCode::L => return game::state::RunState::ShowDropItem, // let-go

            // Main menu
            VirtualKeyCode::Escape => return game::state::RunState::ShowMainMenu,

            VirtualKeyCode::Space => {
                log::debug!("Pausing game ...");
                return game::state::RunState::Paused;
            }
            _ => {
                log::debug!("Got user input: {:?}", key);
                return game::state::RunState::AwaitingInput;
            }
        },
    }
    game::state::RunState::PlayerTurn
}
