use crate::components;
use crate::config;
use log;
use rltk;
use specs::prelude::*;

pub fn spawn_health_potion(ecs: &mut World, pos: components::Position, cfg: &config::Item) {
    let potion_name = cfg.name.clone();
    log::debug!("Creating '{}' at {:?} ...", potion_name, pos);
    ecs.create_entity()
        .with(pos)
        .with(components::Renderable {
            glyph: rltk::to_cp437(cfg.chr),
            fg: rltk::RGB::named(cfg.fg_color),
            bg: rltk::RGB::named(cfg.bg_color),
            render_order: 2,
        })
        .with(components::Name {
            name: potion_name,
        })
        .with(components::Item {})
        .with(components::Consumable{})
        .with(components::ProvidesHealing {
            heal_amount: cfg.power,
        })
        .build();
}
