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
use crate::http::routes::perform_cycle;

pub fn handler(simulation: &mut Simulation, request: Request) {

    let url = request.url();
    let splits: Vec<&str> = url.split("/").collect();
    match splits.get(2) {
        Some(cycle_count) => {
            let cycles: u32 = cycle_count.parse().unwrap();
            for _ in 0..cycles {
                // perform the cycle
                let simulation_result = match simulation.next_cycle() {
                    Some(mut cycle) => {
                        engine::cycle::run(&simulation, &mut cycle);
                        simulation.current_cycle = Some(cycle);
                        Ok(())
                    },
                    None => {
                        Err("Cannot perform another cycle for simulation".to_string())
                    }
                };

                // save the simulation cycle to disk
                let sim_folder_name = format!("./simulations/sim_{}", simulation.simulation_id);
                match &simulation.current_cycle {
                    Some(cycle) => {
                        let mut serialized = cycle.encode_to_vec();
                        info!("full cycle {} size: {}", cycle.cycle_id, serialized.len());
                        let mut compressor = ZlibEncoder::new(Vec::new(), Compression::default());
                        compressor.write_all(&serialized).unwrap();
                        serialized = compressor.finish().unwrap();
                        info!("compressed cycle {} size: {}", cycle.cycle_id, serialized.len());
                        let file_name = format!("{}/cycle_{}.zip", &sim_folder_name[..], cycle.cycle_id);

                        fs::remove_file(&file_name);

                        let mut file = fs::File::create(file_name).expect(&format!("Failed to create cycle file {}", cycle.cycle_id)[..]);
                        file.write_all(&serialized).expect(&format!("Failed to write cycle data {}", cycle.cycle_id)[..]);
                    },
                    None => {}
                };
            }
        },
        None => {
            let response = Response::from_string("invalid request").with_status_code(400);
            request.respond(response).ok();
        }
    };

}