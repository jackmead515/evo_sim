extern crate tiny_http;
extern crate env_logger;
extern crate prost_build;

use std::env;

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

    http::server::start();
}