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
use crate::state::simulator::SimulationMap;
use crate::state::models::*;
use crate::http::server;

pub fn handler(request: Request, simulation_map: Arc<SimulationMap>) {
    info!("Request to perform a cycle recieved");

    let url = request.url();
    let mut splits = url.split("/");

    // safe because the regex guarantees this
    let simulation_id = splits.nth(2).unwrap().parse::<u32>().unwrap();

    // safety retrieve and pop the simulation from the map
    if let Some(mut simulator) = simulation_map.get_pop(&simulation_id) {

         // perform the cycle
        let mut cycle = simulator.next_cycle();
        engine::cycle::run(&simulator.simulation, &mut cycle);
        simulator.current_cycle = Some(cycle);

        // save the simulation cycle to disk
        let sim_folder_name = format!("./simulations/simulation_{}", &simulation_id);
        match &simulator.current_cycle {
            Some(cycle) => {
                let serialized_cycle = cycle.encode_to_vec();
                info!("full cycle {} size: {}", cycle.cycle_id, serialized_cycle.len());
                let mut compressor = ZlibEncoder::new(Vec::new(), Compression::default());
                compressor.write_all(&serialized_cycle).unwrap();
                let serialized = compressor.finish().unwrap();
                info!("compressed cycle {} size: {}", cycle.cycle_id, serialized.len());
                let cycles_folder = format!("{}/cycles", &sim_folder_name[..]);
                let file_name = format!("{}/cycle_{}.zip", cycles_folder, cycle.cycle_id);

                // create the folder for the simulation
                fs::create_dir_all(cycles_folder).unwrap();

                let mut file = fs::OpenOptions::new()
                    .write(true)
                    .create(true)
                    .open(file_name)
                    .unwrap();
                
                file.write_all(&serialized).unwrap();

                // and respond to the user.
                let response = Response::from_data(serialized_cycle).with_status_code(200);
                request.respond(response).ok();
            },
            None => {}
        };

        // insert it back into the map!
        simulation_map.insert(simulation_id, simulator);

    } else {
        let response = Response::from_string(format!("simulation {} not found", simulation_id)).with_status_code(404);
        request.respond(response).ok();
    }
}