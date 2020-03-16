use crate::components;
use crate::config;
use crate::items::potions;
use crate::items::scrolls;
use rltk;
use specs::prelude::*;

pub fn random(ecs: &mut World, start: components::Position, cfg: &config::Items) {
    let roll: i32;
    {
        let mut rng = ecs.write_resource::<rltk::RandomNumberGenerator>();
        roll = rng.roll_dice(1, 5);
    }
    match roll {
        1 => scrolls::spawn_ranged(ecs, start, &cfg.magic_missile_scroll),
        2 => scrolls::spawn_ranged_aoe(ecs, start, &cfg.fireball_scroll),
        3 => scrolls::spawn_ranged_confusion(ecs, start, &cfg.confusion_scroll),
        _ => potions::spawn_health_potion(ecs, start, &cfg.health_potion),
    }
}
