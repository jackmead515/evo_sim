use std::fs;
use std::path;
use std::io::prelude::*;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use prost::Message;
use flate2::Compression;
use flate2::write::ZlibEncoder;
use tiny_http::{Response, Server, Method, Request};
use std::time::{Instant};
use log::{info};
use bytes;

use crate::engine;
use crate::state::simulation::SimulationMap;
use crate::state::models::*;
use crate::http::server;

pub fn handler(request: Request, simulation_map: Arc<SimulationMap>) {
    info!("Request to perform a cycle recieved");

    let url = request.url();
    let mut splits = url.split("/");

    // safe because the regex guarantees this
    let simulation_id = splits.nth(2).unwrap().parse::<u32>().unwrap();

    // safety retrieve and pop the simulation from the map
    if let Some(mut simulation) = simulation_map.get_pop(&simulation_id) {

         // perform the cycle
        let mut cycle = simulation.next_cycle();
        engine::cycle::run(&simulation.simulation, &mut cycle);
        simulation.current_cycle = Some(cycle);

        // and respond to the user.
        let response = Response::from_string("performed cycle").with_status_code(200);
        request.respond(response).ok();

        // save the simulation cycle to disk
        let sim_folder_name = format!("./simulations/simulation_{}", &simulation_id);
        match &simulation.current_cycle {
            Some(cycle) => {
                let mut serialized = cycle.encode_to_vec();
                info!("full cycle {} size: {}", cycle.cycle_id, serialized.len());
                let mut compressor = ZlibEncoder::new(Vec::new(), Compression::default());
                compressor.write_all(&serialized).unwrap();
                serialized = compressor.finish().unwrap();
                info!("compressed cycle {} size: {}", cycle.cycle_id, serialized.len());
                let file_name = format!("{}/cycles/cycle_{}.zip", &sim_folder_name[..], cycle.cycle_id);

                match fs::OpenOptions::new().write(true).open(file_name) {
                    Ok(mut file) => {
                        file.write_all(&serialized)
                            .expect(&format!("Failed to write cycle data {}", cycle.cycle_id)[..]);
                    },
                    Err(_) => {}
                }
            },
            None => {}
        };

        // insert it back into the map!
        simulation_map.insert(simulation_id, simulation);

    } else {
        let response = Response::from_string(format!("simulation {} not found", simulation_id)).with_status_code(404);
        request.respond(response).ok();
    }
}