extern crate dirs;
extern crate log;
extern crate rand;
extern crate rider_config;
extern crate rider_themes;
extern crate serde;
extern crate serde_derive;
extern crate serde_json;
extern crate simplelog;
extern crate uuid;

pub mod config;
pub mod images;
pub mod themes;

fn main() {
    config::create();
    themes::create();
}
