use cfglib;
use serde::Deserialize;
use specs::prelude::*;
use specs_derive::*;
use twyg::LoggerOpts;

const ENV_PREFIX: &str = "EXP";
const CONFIG_FILE: &str = "config";

#[derive(Clone, Debug, Deserialize)]
pub struct Game {
    pub title: String,
    pub welcome: String,    
}

#[derive(Clone, Debug, Deserialize)]
pub struct NPCs {
    pub count: i32,
    pub chr: char,
    pub fg_color: (u8, u8, u8),
    pub bg_color: (u8, u8, u8),
    pub init_x: i32,
    pub init_y: i32,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ViewRange {
    pub tile_count: i32,
}

#[derive(Clone, Copy, Debug, Deserialize)]
pub struct Stats {
    pub max_hp: i32,
    pub starting_hp: i32,
    pub defense: i32,
    pub power: i32,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Player {
    pub name: String,
    pub chr: char,
    pub fg_color: (u8, u8, u8),
    pub bg_color: (u8, u8, u8),
    pub view_range: ViewRange,
    pub stats: Stats,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Monster {
    pub name: String,
    pub chr: char,
    pub stats: Stats,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Monsters {
    pub monster1: Monster,
    pub monster2: Monster,
    pub fg_color: (u8, u8, u8),
    pub bg_color: (u8, u8, u8),
    pub view_range: ViewRange,
}

#[derive(Clone, Copy, Debug, Deserialize)]
pub struct Rooms {
    pub max_count: i32,
    pub min_size: i32,
    pub max_size: i32,
}

#[derive(Clone, Copy, Debug, Deserialize)]
pub struct Map {
    pub fullscreen: bool,
    pub width: i32,
    pub height: i32,
    pub rooms: Rooms,
}

#[derive(Clone, Copy, Debug, Deserialize)]
pub struct TextArea {
    pub height: i32,
}

#[derive(Clone, Component, Debug, Deserialize)]
pub struct AppConfig {
    pub game: Game,
    pub logging: LoggerOpts,
    pub npcs: NPCs,
    pub player: Player,
    pub monsters: Monsters,
    pub map: Map,
    pub text_area: TextArea,
}

impl AppConfig {
    pub fn new() -> Self {
        match new_app_config() {
            Ok(c) => c,
            Err(_) => panic!("Configuration error: check the config file"),
        }
    }
}

pub fn new_app_config() -> Result<AppConfig, cfglib::ConfigError> {
    let mut c = cfglib::Config::new();
    // Start off by merging in the default configuration values
    c.merge(cfglib::File::with_name(CONFIG_FILE))?;
    // Merge in overrides from the environment
    c.merge(cfglib::Environment::with_prefix(ENV_PREFIX))?;
    c.try_into()
}
