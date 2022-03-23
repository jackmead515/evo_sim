
use std::fs;
use std::path;
use std::io::prelude::*;

use prost::Message;
use flate2::Compression;
use flate2::write::ZlibEncoder;
use tiny_http::{Response, Server, Method};
use std::time::{Instant};
use log::{info};
use bytes;

use crate::engine;
use crate::state::simulation::Simulation;
use crate::state::models::*;

use crate::http::routes;

pub fn start() {
    info!("Starting evo sim server");

    let mut simulation = Simulation::new(1);
    let sim_folder_name = format!("./simulations/sim_{}", simulation.simulation_id);
    fs::create_dir_all(&sim_folder_name[..]).expect("Failed to create simulation directory");

    let server = Server::http("0.0.0.0:8000").unwrap();

    for request in server.incoming_requests() {
        let method = request.method();
        let url = request.url();

        info!("Request: {} {}", method, url);

        if matches!(method, Method::Post) && url == "/perform-cycle" {
            routes::perform_cycle::handler(&mut simulation, request);
        } else if matches!(method, Method::Post) && url.starts_with("/perform-cycles") {
            routes::perform_cycles::handler(&mut simulation, request);    
        } else if matches!(method, Method::Put) && url == "/set-parameter" {
            routes::set_parameters::handler(&mut simulation, request);
        } else if matches!(method, Method::Get) && url == "/get-cycle" {
            routes::get_cycle::handler(&mut simulation, request);
        } else if matches!(method, Method::Post) && url.starts_with("/render-cycle") {
            routes::render_cycle::handler(&mut simulation, request);
        } else {
            let response = Response::from_string("invalid request").with_status_code(400);
            request.respond(response).ok();
        }
    }
}