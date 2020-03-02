use crate::components;
use crate::map;
use log;
use rltk::Point;
use specs::prelude::*;
use specs::System;

pub struct MonsterAI {}

impl<'a> System<'a> for MonsterAI {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        WriteExpect<'a, map::Map>,
        ReadExpect<'a, Point>,
        WriteStorage<'a, components::Viewshed>,
        ReadStorage<'a, components::Monster>,
        ReadStorage<'a, components::Name>,
        WriteStorage<'a, components::Position>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut game_map, player_pos, mut viewshed, monster, name, mut position) = data;

        for (mut viewshed, _monster, name, mut pos) in
            (&mut viewshed, &monster, &name, &mut position).join()
        {
            if viewshed.visible_tiles.contains(&*player_pos) {
                log::debug!("{} shouts insults", name.name);
                let path = rltk::a_star_search(
                    game_map.xy_idx(pos.x, pos.y),
                    game_map.xy_idx(player_pos.x, player_pos.y),
                    &mut *game_map,
                );
                if path.success && path.steps.len() > 1 {
                    pos.x = path.steps[1] as i32 % game_map.width;
                    pos.y = path.steps[1] as i32 / game_map.width;
                    viewshed.dirty = true;
                }
            }
        }
    }
}
