use crate::components::{BlocksTile, CombatStats, Monster, Name, Position, Renderable, Viewshed};
use crate::config;
use rltk::{RandomNumberGenerator, RGB};
use specs::prelude::*;

pub fn random(ecs: &mut World, start: Position, cfg: config::Monsters) {
    let roll: i32;
    {
        let mut rng = ecs.write_resource::<RandomNumberGenerator>();
        roll = rng.roll_dice(1, 2);
    }
    match roll {
        1 => spawn(ecs, start, &cfg, &cfg.orc),
        _ => spawn(ecs, start, &cfg, &cfg.goblin),
    }
}

pub fn spawn(ecs: &mut World, start: Position, cfg: &config::Monsters, m: &config::Monster) {
    ecs.create_entity()
        .with(start)
        .with(Renderable {
            glyph: rltk::to_cp437(m.chr),
            fg: RGB::named(cfg.fg_color),
            bg: RGB::named(cfg.bg_color),
            render_order: 1,
        })
        .with(Viewshed {
            visible_tiles: Vec::new(),
            range: cfg.view_range.tile_count.clone(),
            dirty: true,
        })
        .with(Monster {})
        .with(Name {
            name: m.name.clone(),
        })
        .with(BlocksTile {})
        .with(CombatStats {
            max_hp: m.stats.max_hp,
            hp: m.stats.starting_hp,
            defense: m.stats.defense,
            power: m.stats.power,
        })
        .build();
}
