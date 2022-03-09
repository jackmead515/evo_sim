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

pub fn handler(simulation: &mut Simulation, mut request: Request) {
    info!("Request to set a parameter recieved");

    let mut content = String::new();
    request.as_reader().read_to_string(&mut content).unwrap();

    engine::set_parameters();

    let response = Response::from_string("set parameter").with_status_code(200);
    request.respond(response).ok();
}