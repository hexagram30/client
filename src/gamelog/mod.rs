use rltk::prelude::*;
mod builder;
pub use builder::*;
mod logstore;
use logstore::*;
pub use logstore::{clear_log, clone_log, print_log, restore_log};
use serde::{Deserialize, Serialize};
mod events;
pub use events::*;

// XXX maybe move this into logstore ...
#[derive(Serialize, Deserialize, Clone)]
pub struct LogFragment {
    pub color: RGB,
    pub text: String,
}
