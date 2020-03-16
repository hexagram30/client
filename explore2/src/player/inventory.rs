use crate::components;
use crate::game;
use crate::map;
use specs;
use specs::prelude::*;

pub struct ItemCollectionSystem {}

impl<'a> System<'a> for ItemCollectionSystem {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        ReadExpect<'a, Entity>,
        WriteExpect<'a, game::log::GameLog>,
        WriteStorage<'a, components::WantsToPickupItem>,
        WriteStorage<'a, components::Position>,
        ReadStorage<'a, components::Name>,
        WriteStorage<'a, components::InBackpack>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (player_entity, mut gamelog, mut wants_pickup, mut positions, names, mut backpack) =
            data;

        for pickup in wants_pickup.join() {
            positions.remove(pickup.item);
            backpack
                .insert(
                    pickup.item,
                    components::InBackpack {
                        owner: pickup.collected_by,
                    },
                )
                .expect("Unable to insert backpack entry");

            if pickup.collected_by == *player_entity {
                gamelog.entries.push(format!(
                    "You pick up the {}.",
                    names.get(pickup.item).unwrap().name
                ));
            }
        }

        wants_pickup.clear();
    }
}

pub struct ItemUseSystem {}

impl<'a> System<'a> for ItemUseSystem {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        ReadExpect<'a, Entity>,
        WriteExpect<'a, game::log::GameLog>,
        ReadExpect<'a, map::Map>,
        Entities<'a>,
        WriteStorage<'a, components::WantsToUseItem>,
        ReadStorage<'a, components::Name>,
        ReadStorage<'a, components::Consumable>,
        ReadStorage<'a, components::ProvidesHealing>,
        ReadStorage<'a, components::InflictsDamage>,
        WriteStorage<'a, components::CombatStats>,
        WriteStorage<'a, components::SufferDamage>,
        ReadStorage<'a, components::AreaOfEffect>,
        WriteStorage<'a, components::Confusion>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            player_entity,
            mut gamelog,
            game_map,
            entities,
            mut wants_use,
            names,
            consumables,
            healing,
            inflict_damage,
            mut combat_stats,
            mut suffer_damage,
            aoe,
            mut confused,
        ) = data;

        for (entity, useitem) in (&entities, &wants_use).join() {
            let mut used_item = true;

            // Targeting
            let mut targets : Vec<Entity> = Vec::new();
            match useitem.target {
                None => { targets.push( *player_entity ); }
                Some(target) => {
                    let area_effect = aoe.get(useitem.item);
                    match area_effect {
                        None => {
                            // Single target in tile
                            let idx = game_map.xy_idx(target.x, target.y);
                            for mob in game_map.tile_content[idx].iter() {
                                targets.push(*mob);
                            }
                        }
                        Some(area_effect) => {
                            // AoE
                            let mut blast_tiles = rltk::field_of_view(target, area_effect.radius, &*game_map);
                            blast_tiles.retain(|p| p.x > 0 && p.x < game_map.width-1 && p.y > 0 && p.y < game_map.height-1 );
                            for tile_idx in blast_tiles.iter() {
                                let idx = game_map.xy_idx(tile_idx.x, tile_idx.y);
                                for mob in game_map.tile_content[idx].iter() {
                                    targets.push(*mob);
                                }
                            }
                        }
                    }
                }
            }

            // If it heals, apply the healing
            let item_heals = healing.get(useitem.item);
            match item_heals {
                None => {}
                Some(healer) => {
                    used_item = false;
                    for target in targets.iter() {
                        let stats = combat_stats.get_mut(*target);
                        if let Some(stats) = stats {
                            stats.hp = i32::min(stats.max_hp, stats.hp + healer.heal_amount);
                            if entity == *player_entity {
                                gamelog.entries.push(format!("You use the {}, healing {} hp.", names.get(useitem.item).unwrap().name, healer.heal_amount));
                            }
                            used_item = true;
                        }
                    }
                }
            }

            // If it inflicts damage, apply it to the target cell
            let item_damages = inflict_damage.get(useitem.item);
            match item_damages {
                None => {}
                Some(damage) => {
                    used_item = false;
                    for mob in targets.iter() {
                        components::SufferDamage::new_damage(&mut suffer_damage, *mob, damage.damage);
                        if entity == *player_entity {
                            let mob_name = names.get(*mob).unwrap();
                            let item_name = names.get(useitem.item).unwrap();
                            gamelog.entries.push(format!("You use {} on {}, inflicting {} hp.", item_name.name, mob_name.name, damage.damage));
                        }

                        used_item = true;
                    }
                }
            }

            // Can it pass along confusion? Note the use of scopes to escape from the borrow checker!
            let mut add_confusion = Vec::new();
            {
                let causes_confusion = confused.get(useitem.item);
                match causes_confusion {
                    None => {}
                    Some(confusion) => {
                        used_item = false;
                        for mob in targets.iter() {
                            add_confusion.push((*mob, confusion.turns ));
                            if entity == *player_entity {
                                let mob_name = names.get(*mob).unwrap();
                                let item_name = names.get(useitem.item).unwrap();
                                gamelog.entries.push(format!("You use {} on {}, confusing them.", item_name.name, mob_name.name));
                            }
                        }
                    }
                }
            }
            for mob in add_confusion.iter() {
                confused.insert(mob.0, components::Confusion{ turns: mob.1 }).expect("Unable to insert status");
            }

            // If its a consumable, we delete it on use
            if used_item {
                let consumable = consumables.get(useitem.item);
                match consumable {
                    None => {}
                    Some(_) => {
                        entities.delete(useitem.item).expect("Delete failed");
                    }
                }
            }
        }

        wants_use.clear();
    }
}

pub struct ItemDropSystem {}

impl<'a> System<'a> for ItemDropSystem {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        ReadExpect<'a, Entity>,
        WriteExpect<'a, game::log::GameLog>,
        Entities<'a>,
        WriteStorage<'a, components::WantsToDropItem>,
        ReadStorage<'a, components::Name>,
        WriteStorage<'a, components::Position>,
        WriteStorage<'a, components::InBackpack>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            player_entity,
            mut gamelog,
            entities,
            mut wants_drop,
            names,
            mut positions,
            mut backpack,
        ) = data;

        for (entity, to_drop) in (&entities, &wants_drop).join() {
            let mut dropper_pos: components::Position = components::Position { x: 0, y: 0 };
            {
                let dropped_pos = positions.get(entity).unwrap();
                dropper_pos.x = dropped_pos.x;
                dropper_pos.y = dropped_pos.y;
            }
            positions
                .insert(
                    to_drop.item,
                    components::Position {
                        x: dropper_pos.x,
                        y: dropper_pos.y,
                    },
                )
                .expect("Unable to insert position");
            backpack.remove(to_drop.item);

            if entity == *player_entity {
                gamelog.entries.push(format!(
                    "You drop the {}.",
                    names.get(to_drop.item).unwrap().name
                ));
            }
        }

        wants_drop.clear();
    }
}
