use crate::components;
use crate::game;
use specs::prelude::*;
use specs::{self, Join, System};

pub struct MeleeSystem {}

impl<'a> System<'a> for MeleeSystem {
    type SystemData = (
        Entities<'a>,
        WriteExpect<'a, game::log::GameLog>,
        WriteStorage<'a, components::WantsToMelee>,
        ReadStorage<'a, components::Name>,
        ReadStorage<'a, components::CombatStats>,
        WriteStorage<'a, components::SufferDamage>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (entities, mut log, mut wants_melee, names, combat_stats, mut inflict_damage) = data;

        for (_entity, wants_melee, name, stats) in
            (&entities, &wants_melee, &names, &combat_stats).join()
        {
            if stats.hp > 0 {
                let target_stats = combat_stats.get(wants_melee.target).unwrap();
                if target_stats.hp > 0 {
                    let target_name = names.get(wants_melee.target).unwrap();
                    let damage = i32::max(0, stats.power - target_stats.defense);

                    if damage == 0 {
                        log.entries.push(format!(
                            "{} is unable to hurt {}",
                            &name.name, &target_name.name
                        ));
                    } else {
                        log.entries.push(format!(
                            "{} hits {}, for {} hp.",
                            &name.name, &target_name.name, damage
                        ));
                        components::SufferDamage::new_damage(
                            &mut inflict_damage,
                            wants_melee.target,
                            damage,
                        );
                    }
                }
            }
        }

        wants_melee.clear();
    }
}
