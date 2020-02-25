use crate::config;
use log;
use twyg;

pub fn new(cfg: &config::AppConfig) {
    match twyg::setup_logger(&cfg.logging) {
        Ok(_) => {}
        Err(error) => panic!("Could not setup logger: {:?}", error),
    };
    log::info!("Successfully setup logger.");    
}
