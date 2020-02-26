use crate::ai::monster;
use crate::components;
use crate::map;
use crate::physics;
use crate::player;
use rltk::{self, Console, GameState};
use specs::{self, Join, RunNow, WorldExt};
use std::process;

#[derive(PartialEq, Copy, Clone)]
pub enum RunState {
    Paused,
    Running,
    Quitting,
}

pub struct State {
    pub ecs: specs::World,
    pub runstate: RunState,
}

impl State {
    fn run_systems(&mut self) {
        let mut vis = physics::VisibilitySystem {};
        vis.run_now(&self.ecs);
        let mut mob = monster::MonsterAI {};
        mob.run_now(&self.ecs);
        self.ecs.maintain();
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut rltk::Rltk) {
        ctx.cls();
        if self.runstate == RunState::Quitting {
            process::exit(0);
        } else if self.runstate == RunState::Running {
            self.run_systems();
            self.runstate = RunState::Paused;
        } else {
            self.runstate = player::input(self, ctx);
        }

        map::draw(&self.ecs, ctx);

        let positions = self.ecs.read_storage::<components::Position>();
        let renderables = self.ecs.read_storage::<components::Renderable>();
        let game_map = self.ecs.fetch::<map::Map>();

        for (pos, render) in (&positions, &renderables).join() {
            let idx = game_map.xy_idx(pos.x, pos.y);
            if game_map.visible_tiles[idx] {
                ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph)
            }
        }
    }
}
