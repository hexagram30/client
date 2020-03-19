use crate::components;
use crate::config;
use crate::game;
use crate::map;
use rltk::{Point, RGB};
use specs;
use specs::prelude::*;

#[derive(Clone)]
pub struct Character {
    pub location: Point,
    pub entity: Entity,
    
}

pub fn new(cfg: &config::AppConfig, gs: &mut game::state::State, game_map: &map::Map) -> Character {
  log::debug!("Setting up Player ...");
  let (player_x, player_y) = game_map.rooms[0].center();
  Character {
    location: Point::new(player_x, player_y),
    entity: spawn(&mut gs.ecs, components::Position {
      x: player_x,
      y: player_y,
  }, cfg.player.clone()),
  }
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

pub fn get_item(ecs: &mut World) {
  let player_pos = ecs.fetch::<Point>();
  let player_entity = ecs.fetch::<Entity>();
  let entities = ecs.entities();
  let items = ecs.read_storage::<components::Item>();
  let positions = ecs.read_storage::<components::Position>();
  let mut gamelog = ecs.fetch_mut::<game::log::GameLog>();

  let mut target_item: Option<Entity> = None;
  for (item_entity, _item, position) in (&entities, &items, &positions).join() {
    if position.x == player_pos.x && position.y == player_pos.y {
      target_item = Some(item_entity);
    }
  }

  match target_item {
    None => gamelog
      .entries
      .push("There is nothing here to pick up.".to_string()),
    Some(item) => {
      let mut pickup = ecs.write_storage::<components::WantsToPickupItem>();
      pickup
        .insert(
          *player_entity,
          components::WantsToPickupItem {
            collected_by: *player_entity,
            item,
          },
        )
        .expect("Unable to insert want to pickup");
    }
  }
}
