extern crate tiny_http;
extern crate env_logger;
extern crate prost_build;
extern crate serde;
extern crate serde_json;
extern crate bytes;
extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;
extern crate notify;

#[macro_use]
extern crate lazy_static;

use std::env;
use std::sync::{Arc, Mutex};
use crate::state::simulator::{SimulationMap};

pub mod http;
pub mod engine;
pub mod state;

fn main() {
    env::set_var("RUST_LOG", "info");
    env::set_var("OUT_DIR", "src");

    env_logger::init();

    if env::var("BUILD_PROTOS").is_ok() {
        prost_build::compile_protos(
            &["src/models.proto"],
            &["src"]
        ).unwrap();
    }

    let mut simulation_map = SimulationMap::new();
    simulation_map.sync_from_disk();
    simulation_map.watch_changes();

    http::server::start(Arc::new(simulation_map));
}