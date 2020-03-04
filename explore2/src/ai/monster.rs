use crate::components;
use crate::game;
use crate::map;
use rltk;
use rltk::Point;
use specs::{
    self, Entities, Entity, Join, ReadExpect, ReadStorage, System, WriteExpect, WriteStorage,
};

pub struct MonsterAI {}

impl<'a> System<'a> for MonsterAI {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        WriteExpect<'a, map::Map>,
        ReadExpect<'a, Point>,
        ReadExpect<'a, Entity>,
        ReadExpect<'a, game::RunState>,
        Entities<'a>,
        WriteStorage<'a, components::Viewshed>,
        ReadStorage<'a, components::Monster>,
        WriteStorage<'a, components::Position>,
        WriteStorage<'a, components::WantsToMelee>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            mut game_map,
            player_pos,
            player_entity,
            runstate,
            entities,
            mut viewshed,
            monster,
            mut position,
            mut wants_to_melee,
        ) = data;

        if *runstate != game::RunState::MonsterTurn {
            return;
        }

        for (entity, mut viewshed, _monster, mut pos) in
            (&entities, &mut viewshed, &monster, &mut position).join()
        {
            let distance =
                rltk::DistanceAlg::Pythagoras.distance2d(Point::new(pos.x, pos.y), *player_pos);
            if distance < 1.5 {
                wants_to_melee
                    .insert(
                        entity,
                        components::WantsToMelee {
                            target: *player_entity,
                        },
                    )
                    .expect("Unable to insert attack");
            } else if viewshed.visible_tiles.contains(&*player_pos) {
                // Path to the player
                let path = rltk::a_star_search(
                    game_map.xy_idx(pos.x, pos.y),
                    game_map.xy_idx(player_pos.x, player_pos.y),
                    &mut *game_map,
                );
                if path.success && path.steps.len() > 1 {
                    let mut idx = game_map.xy_idx(pos.x, pos.y);
                    game_map.blocked[idx] = false;
                    pos.x = path.steps[1] as i32 % game_map.width;
                    pos.y = path.steps[1] as i32 / game_map.width;
                    idx = game_map.xy_idx(pos.x, pos.y);
                    game_map.blocked[idx] = true;
                    viewshed.dirty = true;
                }
            }
        }
    }
}
