use crate::config;
use crate::game;
use crate::game::persistence;
use rltk::{Console, Rltk, VirtualKeyCode, RGB};

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Selection {
    ContinuePlaying,
    NewGame,
    SaveGame,
    LoadGame,
    Credits,
    Quit,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Result {
    NoSelection { selected: Selection },
    Selected { selected: Selection },
}

pub fn draw(gs: &mut game::state::State, ctx: &mut Rltk) -> Result {
    let save_exists = persistence::file_exists(&gs.ecs);
    let cfg = gs.ecs.fetch::<config::AppConfig>().game.clone();
    let runstate = gs.ecs.fetch::<game::state::RunState>();

    ctx.print_color_centered(
        15,
        RGB::named(rltk::DARK_GREEN),
        RGB::named(rltk::BLACK),
        &cfg.title,
    );

    if let game::state::RunState::MainMenu {
        menu_selection: selection,
    } = *runstate
    {
        if selection == Selection::ContinuePlaying {
            ctx.print_color_centered(
                24,
                RGB::named(rltk::GREEN),
                RGB::named(rltk::BLACK),
                "Return to Game",
            );
        } else {
            ctx.print_color_centered(
                24,
                RGB::named(rltk::WHITE),
                RGB::named(rltk::BLACK),
                "Return to Game",
            );
        }

        if selection == Selection::NewGame {
            ctx.print_color_centered(
                25,
                RGB::named(rltk::GREEN),
                RGB::named(rltk::BLACK),
                "Begin New Game",
            );
        } else {
            ctx.print_color_centered(
                25,
                RGB::named(rltk::WHITE),
                RGB::named(rltk::BLACK),
                "Begin New Game",
            );
        }

        if selection == Selection::SaveGame {
            ctx.print_color_centered(
                26,
                RGB::named(rltk::GREEN),
                RGB::named(rltk::BLACK),
                "Save Game",
            );
        } else {
            ctx.print_color_centered(
                26,
                RGB::named(rltk::WHITE),
                RGB::named(rltk::BLACK),
                "Save Game",
            );
        }

        if save_exists {
            if selection == Selection::LoadGame {
                ctx.print_color_centered(
                    27,
                    RGB::named(rltk::GREEN),
                    RGB::named(rltk::BLACK),
                    "Load Game",
                );
            } else {
                ctx.print_color_centered(
                    27,
                    RGB::named(rltk::WHITE),
                    RGB::named(rltk::BLACK),
                    "Load Game",
                );
            }
        }

        if selection == Selection::Credits {
            ctx.print_color_centered(
                28,
                RGB::named(rltk::GREEN),
                RGB::named(rltk::BLACK),
                "Credits",
            );
        } else {
            ctx.print_color_centered(
                28,
                RGB::named(rltk::WHITE),
                RGB::named(rltk::BLACK),
                "Credits",
            );
        }

        if selection == Selection::Quit {
            ctx.print_color_centered(29, RGB::named(rltk::GREEN), RGB::named(rltk::BLACK), "Quit");
        } else {
            ctx.print_color_centered(29, RGB::named(rltk::WHITE), RGB::named(rltk::BLACK), "Quit");
        }

        match ctx.key {
            None => {
                return Result::NoSelection {
                    selected: selection,
                }
            }
            Some(key) => {
                log::trace!("Got main menu keypress for {:?}", key);
                match key {
                    VirtualKeyCode::Escape => {
                        return Result::NoSelection {
                            selected: Selection::ContinuePlaying,
                        }
                    }
                    VirtualKeyCode::Up => {
                        let mut newselection;
                        match selection {
                            Selection::ContinuePlaying => newselection = Selection::Quit,
                            Selection::NewGame => newselection = Selection::ContinuePlaying,
                            Selection::SaveGame => newselection = Selection::NewGame,
                            Selection::LoadGame => newselection = Selection::SaveGame,
                            Selection::Credits => newselection = Selection::LoadGame,
                            Selection::Quit => newselection = Selection::Credits,
                        }
                        if newselection == Selection::LoadGame && !save_exists {
                            newselection = Selection::NewGame;
                        }
                        return Result::NoSelection {
                            selected: newselection,
                        };
                    }
                    VirtualKeyCode::Down => {
                        let mut newselection;
                        match selection {
                            Selection::ContinuePlaying => newselection = Selection::NewGame,
                            Selection::NewGame => newselection = Selection::SaveGame,
                            Selection::SaveGame => newselection = Selection::LoadGame,
                            Selection::LoadGame => newselection = Selection::Credits,
                            Selection::Credits => newselection = Selection::Quit,
                            Selection::Quit => newselection = Selection::ContinuePlaying,
                        }
                        if newselection == Selection::LoadGame && !save_exists {
                            newselection = Selection::Quit;
                        }
                        return Result::NoSelection {
                            selected: newselection,
                        };
                    }
                    VirtualKeyCode::Return => {
                        return Result::Selected {
                            selected: selection,
                        }
                    }
                    _ => {
                        return Result::NoSelection {
                            selected: selection,
                        }
                    }
                }
            }
        }
    }

    Result::NoSelection {
        selected: Selection::ContinuePlaying,
    }
}
