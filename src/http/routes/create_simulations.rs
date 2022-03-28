use std::fs;
use std::str;
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
use crate::state::simulation::{Simulator, SimulationMap};
use crate::state::models::*;
use crate::http::server;

pub fn handler(mut request: Request, simulation_map: Arc<SimulationMap>) {
    info!("Request to create a simulation recieved");

    // parse the id and folder/file names for the simulation
    let url = request.url();
    let mut splits = url.split("/");
    let simulation_id = splits.nth(2).unwrap().parse::<u32>().unwrap();
    let sim_folder_name = format!("./simulations/simulation_{}", &simulation_id);
    let sim_file_name = format!("./simulations/simulation_{}/simulation.zip", &simulation_id);

    // check if the simulation is in the map or the file already exists on disk
    if !simulation_map.contains(&simulation_id) && !path::Path::new(&sim_folder_name).exists() {

        // create a new simulator object
        let mut simulator = Simulator::new(simulation_id);

        // parse the body as json
        let mut content = String::new();
        request.as_reader().read_to_string(&mut content).unwrap();
        let json = json::parse(&content).unwrap();

        // for each key:value pair, upsert the constants if provided
        for (key, value) in json.entries() {
            if key == "world_width" {
                simulator.simulation.constants.world_width = value.as_u32().unwrap();
            } else if key == "world_height" {
                simulator.simulation.constants.world_height = value.as_u32().unwrap();
            } else if key == "max_steps" {
                simulator.simulation.constants.max_steps = value.as_u32().unwrap();
            } else if key == "creature_amount" {
                simulator.simulation.constants.creature_amount = value.as_u32().unwrap();
            } else if key == "initial_brain_size" {
                simulator.simulation.constants.initial_brain_size = value.as_u32().unwrap();
            } else if key == "max_brain_size" {
                simulator.simulation.constants.max_brain_size = value.as_u32().unwrap();
            } else if key == "min_brain_size" {
                simulator.simulation.constants.min_brain_size = value.as_u32().unwrap();
            } else if key == "brain_input_size" {
                simulator.simulation.constants.brain_input_size = value.as_u32().unwrap();
            } else if key == "brain_output_size" {
                simulator.simulation.constants.brain_output_size = value.as_u32().unwrap();
            } else if key == "initial_block_amount" {
                simulator.simulation.constants.initial_block_amount = value.as_u32().unwrap();
            } else if key == "min_block_amount" {
                simulator.simulation.constants.min_block_amount = value.as_u32().unwrap();
            } else if key == "max_block_amount" {
                simulator.simulation.constants.max_block_amount = value.as_u32().unwrap();
            } else if key == "initial_block_size" {
                simulator.simulation.constants.initial_block_size = value.as_f32().unwrap();
            } else if key == "max_block_size" {
                simulator.simulation.constants.max_block_size = value.as_f32().unwrap();
            } else if key == "min_block_size" {
                simulator.simulation.constants.min_block_size = value.as_f32().unwrap();
            }
        }

        // serialize and compress the simulation bytes
        let mut serialized = simulator.simulation.encode_to_vec();
        let mut compressor = ZlibEncoder::new(Vec::new(), Compression::default());
        compressor.write_all(&serialized).unwrap();
        serialized = compressor.finish().unwrap();

        // create the folder for the simulation
        fs::create_dir_all(sim_folder_name).unwrap();

        // write the dang simulation to disk!
        let mut file = fs::OpenOptions::new()
            .write(true)
            .create(true)
            .open(sim_file_name)
            .unwrap();
        file.write_all(&serialized).unwrap();

        // insert the simulator object into the simulation map
        simulation_map.insert(simulation_id, simulator);

        // and respond to the user.
        request.respond(Response::empty(200)).ok();

    } else {
        let response = Response::from_string("simulation already exists").with_status_code(400);
        request.respond(response).ok();
    }    
}