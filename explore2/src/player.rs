use crate::components;
use crate::config;
use crate::game;
use crate::map;
use log;
use rltk::{Point, Rltk, VirtualKeyCode, RGB};
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
      VirtualKeyCode::Left | VirtualKeyCode::A | VirtualKeyCode::Key4 => {
        try_move(-1, 0, &mut gs.ecs)
      }
      VirtualKeyCode::Right | VirtualKeyCode::D | VirtualKeyCode::Key6 => {
        try_move(1, 0, &mut gs.ecs)
      }
      VirtualKeyCode::Up | VirtualKeyCode::W | VirtualKeyCode::Key8 => try_move(0, -1, &mut gs.ecs),
      VirtualKeyCode::Down | VirtualKeyCode::S | VirtualKeyCode::Key2 => {
        try_move(0, 1, &mut gs.ecs)
      }
      VirtualKeyCode::Key7 | VirtualKeyCode::Q => try_move(-1, -1, &mut gs.ecs),
      VirtualKeyCode::Key9 | VirtualKeyCode::E => try_move(1, -1, &mut gs.ecs),
      VirtualKeyCode::Key3 | VirtualKeyCode::C => try_move(1, 1, &mut gs.ecs),
      VirtualKeyCode::Key1 | VirtualKeyCode::Z => try_move(-1, 1, &mut gs.ecs),
      VirtualKeyCode::Space => {
        log::debug!("Pausing game ...");
        return game::state::RunState::Paused;
      }
      VirtualKeyCode::Escape => {
        log::info!("Quitting ...");
        return game::state::RunState::Quitting;
      }
      _ => {
        log::debug!("Got user input: {:?}", key);
        return game::state::RunState::AwaitingInput;
      }
    },
  }
  game::state::RunState::PlayerTurn
}

/// Spawns the player and returns his/her entity object.
pub fn spawn(ecs: &mut World, start: components::Position, cfg: config::Player) -> Entity {
  ecs
    .create_entity()
    .with(start)
    .with(components::Renderable {
      glyph: rltk::to_cp437(cfg.chr),
      fg: RGB::named(cfg.fg_color),
      bg: RGB::named(cfg.bg_color),
      render_order: 0,
    })
    .with(components::Player {})
    .with(components::Viewshed {
      visible_tiles: Vec::new(),
      range: cfg.view_range.tile_count,
      dirty: true,
    })
    .with(components::Name {
      name: cfg.name.clone(),
    })
    .with(components::CombatStats {
      max_hp: cfg.stats.max_hp,
      hp: cfg.stats.starting_hp,
      defense: cfg.stats.defense,
      power: cfg.stats.power,
    })
    .build()
}
