use crate::ai::monster;
use crate::combat;
use crate::components;
use crate::gui;
use crate::map;
use crate::physics;
use crate::player;
use rltk::{self, Console, GameState};
use specs::{self, Join, RunNow, WorldExt};
use std::process;

#[derive(PartialEq, Copy, Clone)]
pub enum RunState {
    AwaitingInput,
    PreRun,
    PlayerTurn,
    MonsterTurn,
    Paused,
    Quitting,
}

pub struct State {
    pub ecs: specs::World,
}

impl State {
    fn run_systems(&mut self) {
        let mut vis = physics::VisibilitySystem {};
        vis.run_now(&self.ecs);
        let mut mob = monster::MonsterAI {};
        mob.run_now(&self.ecs);
        let mut mapindex = map::IndexingSystem {};
        mapindex.run_now(&self.ecs);
        let mut melee = combat::melee::MeleeSystem {};
        melee.run_now(&self.ecs);
        let mut damage = combat::damage::DamageSystem {};
        damage.run_now(&self.ecs);
        self.ecs.maintain();
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut rltk::Rltk) {
        ctx.cls();
        let mut newrunstate;
        {
            let runstate = self.ecs.fetch::<RunState>();
            newrunstate = *runstate;
        }

        match newrunstate {
            RunState::PreRun => {
                self.run_systems();
                newrunstate = RunState::AwaitingInput;
            }
            RunState::AwaitingInput => {
                newrunstate = player::input(self, ctx);
            }
            RunState::PlayerTurn => {
                self.run_systems();
                newrunstate = RunState::MonsterTurn;
            }
            RunState::MonsterTurn => {
                self.run_systems();
                newrunstate = RunState::AwaitingInput;
            }
            RunState::Paused => {
                self.run_systems();
                newrunstate = RunState::AwaitingInput;
            }
            RunState::Quitting => {
                process::exit(0);
            }
        }

        {
            let mut runwriter = self.ecs.write_resource::<RunState>();
            *runwriter = newrunstate;
        }
        combat::damage::delete_the_dead(&mut self.ecs);

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
        gui::draw(&self.ecs, ctx);
    }
}
