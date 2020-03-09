use crate::components;
use crate::config;
use crate::game;
use crate::map;
use rltk::{Console, Point, Rltk, RGB};
use specs;
use specs::prelude::*;

#[derive(Default)]
pub struct TextArea {
    pub height: i32,
}

#[derive(Default)]
pub struct MapArea {
    pub width: i32,
    pub height: i32,
}

#[derive(Default)]
pub struct GUI {
    pub map_area: MapArea,
    pub text_area: TextArea,
    pub width: i32,
    pub height: i32,
}

pub fn new(cfg: &config::AppConfig) -> GUI {
    let text_area = TextArea {
        height: cfg.text_area.height,
    };
    let map_area = MapArea {
        width: cfg.map.width,
        height: cfg.map.height,
    };
    GUI {
        map_area: map_area,
        text_area: text_area,
        width: cfg.map.width,
        height: cfg.map.height + cfg.text_area.height,
    }
}

pub fn draw(ecs: &World, ctx: &mut Rltk) {
    let gui = ecs.fetch::<GUI>();
    ctx.draw_box(
        0,
        gui.map_area.height - 1,
        gui.width - 1,
        gui.text_area.height,
        RGB::named(rltk::WHITE),
        RGB::named(rltk::BLACK),
    );

    let combat_stats = ecs.read_storage::<components::CombatStats>();
    let players = ecs.read_storage::<components::Player>();
    for (_player, stats) in (&players, &combat_stats).join() {
        let health = format!(" HP: {} / {} ", stats.hp, stats.max_hp);
        // XXX let's put this 12 into config ... need to look up API usage
        ctx.print_color(
            12,
            gui.map_area.height - 1,
            RGB::named(rltk::YELLOW),
            RGB::named(rltk::BLACK),
            &health,
        );
        // XXX let's put the 28 and 51 into config ... need to look up API usage
        ctx.draw_bar_horizontal(
            28,
            gui.map_area.height - 1,
            51,
            stats.hp,
            stats.max_hp,
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

    // XXX This isn't working for the current layout ...
    // Draw mouse cursor
    // let mouse_pos = ctx.mouse_pos();
    // ctx.set_bg(mouse_pos.0, mouse_pos.1, RGB::named(rltk::MAGENTA));
    // draw_tooltips(ecs, ctx);
}

// XXX This isn't working for the current layout ...
fn _draw_tooltips(ecs: &World, ctx: &mut Rltk) {
    let map = ecs.fetch::<map::Map>();
    let names = ecs.read_storage::<components::Name>();
    let positions = ecs.read_storage::<components::Position>();

    let mouse_pos = ctx.mouse_pos();
    if mouse_pos.0 >= map.width || mouse_pos.1 >= map.height {
        return;
    }
    let mut tooltip: Vec<String> = Vec::new();
    for (name, position) in (&names, &positions).join() {
        if position.x == mouse_pos.0 && position.y == mouse_pos.1 {
            tooltip.push(name.name.to_string());
        }
    }

    if !tooltip.is_empty() {
        let mut width: i32 = 0;
        for s in tooltip.iter() {
            if width < s.len() as i32 {
                width = s.len() as i32;
            }
        }
        width += 3;

        if mouse_pos.0 > 40 {
            let arrow_pos = Point::new(mouse_pos.0 - 2, mouse_pos.1);
            let left_x = mouse_pos.0 - width;
            let mut y = mouse_pos.1;
            for s in tooltip.iter() {
                ctx.print_color(
                    left_x,
                    y,
                    RGB::named(rltk::WHITE),
                    RGB::named(rltk::GREY),
                    s,
                );
                let padding = (width - s.len() as i32) - 1;
                for i in 0..padding {
                    ctx.print_color(
                        arrow_pos.x - i,
                        y,
                        RGB::named(rltk::WHITE),
                        RGB::named(rltk::GREY),
                        &" ".to_string(),
                    );
                }
                y += 1;
            }
            ctx.print_color(
                arrow_pos.x,
                arrow_pos.y,
                RGB::named(rltk::WHITE),
                RGB::named(rltk::GREY),
                &"->".to_string(),
            );
        } else {
            let arrow_pos = Point::new(mouse_pos.0 + 1, mouse_pos.1);
            let left_x = mouse_pos.0 + 3;
            let mut y = mouse_pos.1;
            for s in tooltip.iter() {
                ctx.print_color(
                    left_x + 1,
                    y,
                    RGB::named(rltk::WHITE),
                    RGB::named(rltk::GREY),
                    s,
                );
                let padding = (width - s.len() as i32) - 1;
                for i in 0..padding {
                    ctx.print_color(
                        arrow_pos.x + 1 + i,
                        y,
                        RGB::named(rltk::WHITE),
                        RGB::named(rltk::GREY),
                        &" ".to_string(),
                    );
                }
                y += 1;
            }
            ctx.print_color(
                arrow_pos.x,
                arrow_pos.y,
                RGB::named(rltk::WHITE),
                RGB::named(rltk::GREY),
                &"<-".to_string(),
            );
        }
    }
}
