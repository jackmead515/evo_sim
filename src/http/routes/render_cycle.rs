use std::fs;
use std::path;
use std::io::prelude::*;

use prost::Message;
use flate2::Compression;
use flate2::write::ZlibEncoder;
use tiny_http::{Response, Server, Method, Request};
use std::time::{Instant};
use log::{info};
use bytes;

use crate::engine;
use crate::state::simulation::Simulation;
use crate::state::models::*;

pub fn handler(simulation: &mut Simulation, request: Request) {
    info!("Request to render a cycle");
    let sim_folder_name = format!("./simulations/sim_{}", simulation.simulation_id);

    let url = request.url();
    let splits: Vec<&str> = url.split("/").collect();
    match splits.get(2) {
        Some(cycle_id) => {
            let file_name = format!("{}/cycle_{}.zip", &sim_folder_name[..], cycle_id);
            if path::Path::new(&file_name).exists() {

                let mut file = fs::File::open(file_name).unwrap();
                let mut buffer = Vec::new();
                file.read_to_end(&mut buffer).unwrap();

                let cycle = Cycle::decode(bytes::Bytes::from(buffer)).unwrap();
                engine::render::run(&simulation, &cycle);
            }
        },
        None => {
            let response = Response::from_string("invalid request").with_status_code(400);
            request.respond(response).ok();
        }
    };
}