use crate::components;
use crate::config;
use crate::game;
use crate::gui::tooltips;
use crate::map;
use rltk::{Console, Rltk, RGB};
use specs;
use specs::prelude::*;

#[derive(Default)]
pub struct GUI {
    pub map_area: config::MapArea,
    pub text_area: config::TextArea,
    pub width: i32,
    pub height: i32,
    pub fg_color: (u8, u8, u8),
    pub bg_color: (u8, u8, u8),
    pub cursor_color: (u8, u8, u8),
}

pub fn new(cfg: &config::Gui) -> GUI {
    GUI {
        map_area: cfg.map_area,
        text_area: cfg.text_area,
        width: cfg.map_area.width,
        height: cfg.map_area.height + cfg.text_area.height,
        fg_color: cfg.fg_color,
        bg_color: cfg.bg_color,
        cursor_color: cfg.cursor_color,
    }
}

pub fn draw(ecs: &World, ctx: &mut Rltk) {
    let gui = ecs.fetch::<GUI>();
    ctx.draw_box(
        0,
        gui.map_area.height - 1,
        gui.width - 1,
        gui.text_area.height,
        RGB::named(gui.fg_color),
        RGB::named(gui.bg_color),
    );

    let game_map = ecs.fetch::<map::Map>();
    let depth = format!("Level depth: {}", game_map.depth);
    // XXX Can we put these numeric values in config?
    ctx.print_color(
        2,
        gui.map_area.height - 1,
        RGB::named(rltk::YELLOW),
        RGB::named(rltk::BLACK),
        &depth,
    );

    let combat_stats = ecs.read_storage::<components::CombatStats>();
    let players = ecs.read_storage::<components::Player>();
    for (_player, stats) in (&players, &combat_stats).join() {
        let health = format!(" HP: {} / {} ", stats.hp, stats.max_hp);
        // XXX let's calculate the column positions instead of hard-coding them
        ctx.print_color(
            20,
            gui.map_area.height - 1,
            // XXX add colors to config
            RGB::named(rltk::YELLOW),
            RGB::named(rltk::BLACK),
            &health,
        );
        // XXX let's calculate the column positions instead of hard-coding them
        ctx.draw_bar_horizontal(
            36,
            gui.map_area.height - 1,
            59,
            stats.hp,
            stats.max_hp,
            // XXX add colors to config
            RGB::named(rltk::GREEN),
            RGB::named(rltk::BLACK),
        );
    }

    // Render log messages
    let log = ecs.fetch::<game::log::GameLog>();
    let mut y = gui.map_area.height;
    for e in log.entries.iter().rev() {
        if y < gui.height - 1 {
            ctx.print(2, y, e);
        }
        y += 1;
    }

    // Draw mouse cursor
    let mouse_pos = ctx.mouse_pos();
    ctx.set_bg(mouse_pos.0, mouse_pos.1, RGB::named(gui.cursor_color));
    tooltips::draw(ecs, ctx);
}
