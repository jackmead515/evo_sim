
use std::fs;
use std::io::prelude::*;

use prost::Message;
use flate2::Compression;
use flate2::write::ZlibEncoder;
use tiny_http::{Response, Server, Method};
use std::time::{Instant};
use log::{info};

use crate::engine;
use crate::state::simulation::Simulation;

pub fn handle_perform_cycle(simulation: &mut Simulation) -> Result<(), String> {
    match simulation.next_cycle() {
        Some(mut cycle) => {
            engine::perform_cycle(&simulation, &mut cycle);
            simulation.current_cycle = Some(cycle);
            return Ok(());
        },
        None => {
            return Err("Cannot perform another cycle for simulation".to_string());
        }
    }; 
}

pub fn start() {
    info!("Starting evo sim server");

    let mut simulation = Simulation::new(1);

    let sim_folder_name = format!("./simulations/sim_{}", simulation.simulation_id);

    fs::create_dir_all(&sim_folder_name[..]).expect("Failed to create simulation directory");

    let server = Server::http("0.0.0.0:8000").unwrap();

    for mut request in server.incoming_requests() {
        let method = request.method();
        let url = request.url();

        info!("Request: {} {}", method, url);

        if matches!(method, Method::Post) && url == "/perform-cycle" {
            info!("Request to perform a cycle recieved");
            
            // perform the cycle and respond to the user.
            let now = Instant::now();
            match handle_perform_cycle(&mut simulation) {
                Ok(_) => {
                    let response = Response::from_string("performed cycle").with_status_code(200);
                    request.respond(response).ok();
                },
                Err(message) => {
                    let response = Response::from_string(message).with_status_code(400);
                    request.respond(response).ok();
                }
            }

            // save the simulation cycle to disk
            match &simulation.current_cycle {
                Some(cycle) => {
                    let mut serialized = cycle.encode_to_vec();
                    info!("full cycle {} size: {}", cycle.cycle_id, serialized.len());
                    let mut compressor = ZlibEncoder::new(Vec::new(), Compression::default());
                    compressor.write_all(&serialized).unwrap();
                    serialized = compressor.finish().unwrap();
                    info!("compressed cycle {} size: {}", cycle.cycle_id, serialized.len());
                    let file_name = format!("{}/cycle_{}.zip", &sim_folder_name[..], cycle.cycle_id);
                    let mut file = fs::File::create(file_name).expect(&format!("Failed to create cycle file {}", cycle.cycle_id)[..]);
                    file.write_all(&serialized).expect(&format!("Failed to write cycle data {}", cycle.cycle_id)[..]);
                },
                None => {}
            };
            info!("/perform-cycle {} ms", now.elapsed().as_millis());

        } else if matches!(method, Method::Put) && url == "/set-parameter" {
            info!("Request to set a parameter recieved");

            let mut content = String::new();
            request.as_reader().read_to_string(&mut content).unwrap();

            engine::set_parameters();

            let response = Response::from_string("set parameter").with_status_code(200);
            request.respond(response).ok();

        } else if matches!(method, Method::Get) && url == "/get-cycle" {
            info!("Request to get a cycle recieved");

            let mut content = String::new();
            request.as_reader().read_to_string(&mut content).unwrap();

            let cycle = engine::get_cycle();

            let response = Response::from_string("get cycle").with_status_code(200);
            request.respond(response).ok();

        } else {
            let response = Response::from_string("invalid request").with_status_code(400);
            request.respond(response).ok();

        }
    }
}