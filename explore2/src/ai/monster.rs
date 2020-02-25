use crate::components;
use log;
use rltk::Point;
use specs::prelude::*;
use specs::System;

pub struct MonsterAI {}

impl<'a> System<'a> for MonsterAI {
    type SystemData = (
        ReadExpect<'a, Point>,
        ReadStorage<'a, components::Viewshed>,
        ReadStorage<'a, components::Monster>,
        ReadStorage<'a, components::Name>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (player_pos, viewshed, monster, name) = data;

        for (viewshed, _monster, name) in (&viewshed, &monster, &name).join() {
            if viewshed.visible_tiles.contains(&*player_pos) {
                log::debug!("{} shouts insults", name.name);
            }
        }
    }
}
