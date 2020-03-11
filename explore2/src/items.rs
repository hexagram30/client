use crate::components;
use crate::config;
use log;
use rltk;
use specs::prelude::*;

pub fn random(ecs: &mut World, start: components::Position, cfg: &config::Items) {
    // XXX once we have more than one item, we can uncomment this
    // let roll: i32;
    // {
    //     let mut rng = ecs.write_resource::<rltk::RandomNumberGenerator>();
    //     roll = rng.roll_dice(1, 2);
    // }
    // match roll {
    //     1 => spawn_health_potion(ecs, start, &cfg),
    //     _ => spawn_other_thing(ecs, start, &cfg),
    // }
    // XXX until then:
    spawn_health_potion(ecs, start, cfg);
}

fn spawn_health_potion(ecs: &mut World, pos: components::Position, cfg: &config::Items) {
    log::debug!("Creating health potion at {:?} ...", pos);
    ecs.create_entity()
        .with(pos)
        .with(components::Renderable {
            glyph: rltk::to_cp437(cfg.health_potion.chr),
            fg: rltk::RGB::named(cfg.fg_color),
            bg: rltk::RGB::named(cfg.bg_color),
            render_order: 2,
        })
        .with(components::Name {
            name: cfg.health_potion.name.clone(),
        })
        .with(components::Item {})
        .with(components::Potion {
            heal_amount: cfg.health_potion.hp,
        })
        .build();
}
