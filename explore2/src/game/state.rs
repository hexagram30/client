use crate::ai::monster;
use crate::combat;
use crate::components;
use crate::game::persistence;
use crate::gui;
use crate::gui::menus;
use crate::map;
use crate::physics;
use crate::player;
use log;
use rltk::{self, Console, GameState};
use specs::{self, Entity, Join, RunNow, WorldExt};

#[derive(PartialEq, Copy, Clone)]
pub enum RunState {
    AwaitingInput,
    PreRun,
    PlayerTurn,
    MonsterTurn,
    Paused,
    Quitting,
    ShowDropItem,
    ShowInventory,
    ShowTargeting { range : i32, item : Entity},
    ShowMainMenu,
    MainMenu { menu_selection : menus::main::Selection },
    StartNewGame,
    SaveGame,
    LoadGame,
    ShowCredits,
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
        let mut pickup = player::inventory::ItemCollectionSystem {};
        pickup.run_now(&self.ecs);
        let mut itemuse = player::inventory::ItemUseSystem {};
        itemuse.run_now(&self.ecs);
        let mut drop_items = player::inventory::ItemDropSystem {};
        drop_items.run_now(&self.ecs);

        self.ecs.maintain();
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut rltk::Rltk) {
        let mut newrunstate;
        {
            let runstate = self.ecs.fetch::<RunState>();
            newrunstate = *runstate;
        }

        ctx.cls();

        match newrunstate {
            RunState::MainMenu{..} => {}
            _ => {
                map::draw(&self.ecs, ctx);

                {
                    let positions = self.ecs.read_storage::<components::Position>();
                    let renderables = self.ecs.read_storage::<components::Renderable>();
                    let game_map = self.ecs.fetch::<map::Map>();

                    let mut data = (&positions, &renderables).join().collect::<Vec<_>>();
                    data.sort_by(|&a, &b| b.1.render_order.cmp(&a.1.render_order) );
                    for (pos, render) in data.iter() {
                        let idx = game_map.xy_idx(pos.x, pos.y);
                        if game_map.visible_tiles[idx] { ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph) }
                    }

                    gui::draw(&self.ecs, ctx);
                }
            }
        }

        match newrunstate {
            RunState::PreRun => {
                self.run_systems();
                self.ecs.maintain();
                newrunstate = RunState::AwaitingInput;
            }
            RunState::AwaitingInput => {
                newrunstate = player::user::input(self, ctx);
            }
            RunState::PlayerTurn => {
                self.run_systems();
                self.ecs.maintain();
                newrunstate = RunState::MonsterTurn;
            }
            RunState::MonsterTurn => {
                self.run_systems();
                self.ecs.maintain();
                newrunstate = RunState::AwaitingInput;
            }
            RunState::ShowInventory => {
                let result = menus::item::show_inventory(self, ctx);
                match result.0 {
                    menus::item::Result::Cancel => newrunstate = RunState::AwaitingInput,
                    menus::item::Result::NoResponse => {}
                    menus::item::Result::Selected => {
                        let item_entity = result.1.unwrap();
                        let is_ranged = self.ecs.read_storage::<components::Ranged>();
                        let is_item_ranged = is_ranged.get(item_entity);
                        if let Some(is_item_ranged) = is_item_ranged {
                            newrunstate = RunState::ShowTargeting{ 
                                range: is_item_ranged.range, 
                                item: item_entity };
                        } else {
                            let mut intent = self.ecs.write_storage::<components::WantsToUseItem>();
                            intent.insert(*self.ecs.fetch::<Entity>(), components::WantsToUseItem{ 
                                item: item_entity, target: None }).expect("Unable to insert intent");
                            newrunstate = RunState::PlayerTurn;
                        }
                    }
                }
            }
            RunState::ShowDropItem => {
                let result = menus::item::drop(self, ctx);
                match result.0 {
                    menus::item::Result::Cancel => newrunstate = RunState::AwaitingInput,
                    menus::item::Result::NoResponse => {}
                    menus::item::Result::Selected => {
                        let item_entity = result.1.unwrap();
                        let mut intent = self.ecs.write_storage::<components::WantsToDropItem>();
                        intent
                            .insert(
                                *self.ecs.fetch::<Entity>(),
                                components::WantsToDropItem { item: item_entity },
                            )
                            .expect("Unable to insert intent");
                        newrunstate = RunState::PlayerTurn;
                    }
                }
            }
            RunState::ShowTargeting{range, item} => {
                let result = menus::target::ranged(self, ctx, range);
                match result.0 {
                    menus::item::Result::Cancel => newrunstate = RunState::AwaitingInput,
                    menus::item::Result::NoResponse => {}
                    menus::item::Result::Selected => {
                        let mut intent = self.ecs.write_storage::<components::WantsToUseItem>();
                        intent.insert(*self.ecs.fetch::<Entity>(), components::WantsToUseItem{ 
                            item, target: result.1 }).expect("Unable to insert intent");
                        newrunstate = RunState::PlayerTurn;
                    }
                }
            }
            RunState::ShowMainMenu => {
                newrunstate = RunState::MainMenu{ menu_selection: menus::main::Selection::ContinuePlaying };
            }
            RunState::MainMenu{ .. } => {
                let result = menus::main::draw(self, ctx);
                match result {
                    menus::main::Result::NoSelection{ selected } => newrunstate = RunState::MainMenu{ menu_selection: selected },
                    menus::main::Result::Selected{ selected } => {
                        log::debug!("Handling keypress for {:?}", selected);
                        match selected {
                            menus::main::Selection::ContinuePlaying => newrunstate = RunState::AwaitingInput,
                            menus::main::Selection::NewGame => newrunstate = RunState::StartNewGame,
                            menus::main::Selection::SaveGame => newrunstate = RunState::SaveGame,
                            menus::main::Selection::LoadGame => newrunstate = RunState::LoadGame,
                            menus::main::Selection::Credits => newrunstate = RunState::ShowCredits,
                            menus::main::Selection::Quit => newrunstate = RunState::Quitting,
                        }
                    }
                }
            }
            RunState::StartNewGame => {
                log::info!("Starting a new game is not yet implemented");
                // XXX we can't do the following until we can essentially
                // replace all the components properly ... re-running the 
                // main_loop here is just going to make the game hang ;-)
                // log::info!("Starting new game ...");
                // let cfg = config::AppConfig::new();
                // world::setup(cfg);

                // XXX this doesn't work either
                // let cfg = config::AppConfig::new();
                // let mut gs = State {
                //     ecs: specs::World::new(),
                // };
                // world::setup(cfg, &mut gs);

                newrunstate = RunState::PreRun;
            }
            RunState::SaveGame => {
                log::info!("Saving game ...");
                persistence::save(&mut self.ecs);
                newrunstate = RunState::AwaitingInput;
                log::info!("Saved.");
            }
            RunState::LoadGame => {
                log::info!("Loading game ...");
                persistence::load(&mut self.ecs);
                newrunstate = RunState::AwaitingInput;
                // persistence::delete(&self.ecs);
                log::info!("Game loaded.");
            }
            RunState::ShowCredits => {
                log::info!("Credits screen not yet implemented");
                newrunstate = RunState::AwaitingInput;
            }
            RunState::Paused => {
                self.run_systems();
                newrunstate = RunState::AwaitingInput;
            }
            RunState::Quitting => {
                log::info!("Quitting ...");
                ctx.quit();
            }
        }

        {
            let mut runwriter = self.ecs.write_resource::<RunState>();
            *runwriter = newrunstate;
        }
        combat::damage::delete_the_dead(&mut self.ecs);
    }
}
