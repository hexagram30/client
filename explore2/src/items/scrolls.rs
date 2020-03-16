use crate::components;
use crate::config;
use log;
use rltk;
use specs::prelude::*;

pub fn spawn_ranged(ecs: &mut World, pos: components::Position, cfg: &config::Item) {
    let scroll_name = cfg.name.clone();
    log::debug!("Creating '{}' at {:?} ...", scroll_name, pos);
    ecs.create_entity()
        .with(pos)
        .with(components::Renderable {
            glyph: rltk::to_cp437(cfg.chr),
            fg: rltk::RGB::named(cfg.fg_color),
            bg: rltk::RGB::named(cfg.bg_color),
            render_order: 2,
        })
        .with(components::Name {
            name: scroll_name,
        })
        .with(components::Item {})
        .with(components::Consumable{})
        .with(components::Ranged{ range: cfg.range.unwrap() })
        .with(components::InflictsDamage{ damage: cfg.power })
        .build();
}

pub fn spawn_ranged_aoe(ecs: &mut World, pos: components::Position, cfg: &config::Item) {
    let scroll_name = cfg.name.clone();
    log::debug!("Creating '{}' at {:?} ...", scroll_name, pos);
    ecs.create_entity()
        .with(pos)
        .with(components::Renderable {
            glyph: rltk::to_cp437(cfg.chr),
            fg: rltk::RGB::named(cfg.fg_color),
            bg: rltk::RGB::named(cfg.bg_color),
            render_order: 2,
        })
        .with(components::Name {
            name: scroll_name,
        })
        .with(components::Item {})
        .with(components::Consumable{})
        .with(components::Ranged{ range: cfg.range.unwrap() })
        .with(components::InflictsDamage{ damage: cfg.power })
        .with(components::AreaOfEffect{ radius: cfg.radius.unwrap() })
        .build();
}

pub fn spawn_ranged_confusion(ecs: &mut World, pos: components::Position, cfg: &config::Item) {
    let scroll_name = cfg.name.clone();
    log::debug!("Creating '{}' at {:?} ...", scroll_name, pos);
    ecs.create_entity()
        .with(pos)
        .with(components::Renderable {
            glyph: rltk::to_cp437(cfg.chr),
            fg: rltk::RGB::named(cfg.fg_color),
            bg: rltk::RGB::named(cfg.bg_color),
            render_order: 2,
        })
        .with(components::Name {
            name: scroll_name,
        })
        .with(components::Item {})
        .with(components::Consumable{})
        .with(components::Ranged{ range: cfg.range.unwrap() })
        .with(components::Confusion{ turns: cfg.power })
        .build();
}
