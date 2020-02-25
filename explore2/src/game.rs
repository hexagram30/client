use rltk::{self, Console, GameState};
use crate::components;
use crate::map;
use crate::physics;
use crate::player;
use specs::{self, Join, RunNow, WorldExt};

pub struct State {
    pub ecs: specs::World,
}

impl State {
    fn run_systems(&mut self) {
        let mut vis = physics::VisibilitySystem {};
        vis.run_now(&self.ecs);
        self.ecs.maintain();
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut rltk::Rltk) {
        ctx.cls();

        player::input(self, ctx);
        self.run_systems();

        map::draw(&self.ecs, ctx);

        let positions = self.ecs.read_storage::<components::Position>();
        let renderables = self.ecs.read_storage::<components::Renderable>();

        for (pos, render) in (&positions, &renderables).join() {
            ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph);
        }
    }
}
