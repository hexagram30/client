use crate::components;
use crate::config;
use crate::items;
use crate::monster;
use crate::rect;
use log;
use rltk;
use specs::prelude::*;

/// Fills a room with stuff!
pub fn spawn(ecs: &mut World, room: &rect::Rect, cfg: &config::AppConfig) {
    let mut monster_spawn_points: Vec<usize> = Vec::new();
    let mut item_spawn_points: Vec<usize> = Vec::new();

    // Scope to keep the borrow checker happy
    {
        let mut rng = ecs.write_resource::<rltk::RandomNumberGenerator>();
        let num_monsters = rng.roll_dice(1, cfg.rooms.max_monsters + 2) - 3;
        let num_items = rng.roll_dice(1, cfg.rooms.max_items + 2) - 1;

        log::debug!("Calculating monster locations ...");
        for _i in 0..num_monsters {
            let mut added = false;
            while !added {
                let x = (room.x1 + rng.roll_dice(1, i32::abs(room.x2 - room.x1))) as usize;
                let y = (room.y1 + rng.roll_dice(1, i32::abs(room.y2 - room.y1))) as usize;
                let idx = (y * cfg.gui.map_area.width as usize) + x;
                if !monster_spawn_points.contains(&idx) {
                    monster_spawn_points.push(idx);
                    added = true;
                }
            }
        }

        log::debug!("Calculating item locations ...");
        for _i in 0..num_items {
            let mut added = false;
            while !added {
                let x = (room.x1 + rng.roll_dice(1, i32::abs(room.x2 - room.x1))) as usize;
                let y = (room.y1 + rng.roll_dice(1, i32::abs(room.y2 - room.y1))) as usize;
                let idx = (y * cfg.gui.map_area.width as usize) + x;
                if !item_spawn_points.contains(&idx) {
                    item_spawn_points.push(idx);
                    added = true;
                }
            }
        }
    }

    log::debug!("Checking to see if room gets monsters ...");
    for idx in monster_spawn_points.iter() {
        let x = *idx as i32 % cfg.gui.map_area.width;
        let y = *idx as i32 / cfg.gui.map_area.width;
        let pos = components::Position { x: x, y: y };
        monster::random(ecs, pos, &cfg.monsters);
    }

    log::debug!("Checking to see if room gets items ...");
    for idx in item_spawn_points.iter() {
        let x = *idx as i32 % cfg.gui.map_area.width;
        let y = *idx as i32 / cfg.gui.map_area.width;
        let pos = components::Position { x: x, y: y };
        items::random(ecs, pos, &cfg.items);
    }
}
