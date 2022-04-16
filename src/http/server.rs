
use std::fs;
use std::path;
use std::error::Error;
use std::io::prelude::*;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;

use prost::Message;
use flate2::Compression;
use flate2::write::ZlibEncoder;
use tiny_http::{Response, Request, Server, Method};
use std::time::{Instant};
use log::{info};
use regex::Regex;
use lazy_static;

use crate::http::routes;
use crate::state::simulator::{SimulationMap};

lazy_static! {
    static ref SIMULATIONS_REGEX: Regex = {
        Regex::new(r"^/simulations$").unwrap()
    };
    static ref GET_SIMULATIONS_REGEX: Regex = {
        Regex::new(r"^/simulations/\d{1,4}$").unwrap()
    };
    static ref CYCLES_REGEX: Regex = {
        Regex::new(r"^/simulations/\d{1,4}/cycles$").unwrap()
    };
    static ref GET_CYCLES_REGEX: Regex = {
        Regex::new(r"^/simulations/\d{1,4}/cycles/\d{1,4}$").unwrap()
    };
}

pub fn start(simulation_map: Arc<SimulationMap>) {
    info!("Starting evo sim server");

    let server = Arc::new(Server::http("0.0.0.0:8000").expect("Failed to create http server"));
    let mut handlers = Vec::new();

    for _ in 0..4 {
        let server = server.clone();
        let sim_map = simulation_map.clone();
        handlers.push(thread::spawn(move || {
            for request in server.incoming_requests() {
                let now = Instant::now();
                let map = sim_map.clone();
                let method = request.method();
                let url = request.url();
                let req_url = format!("{} {}", method, url);

                info!("incoming request: {}", req_url);

                if matches!(method, Method::Post) && CYCLES_REGEX.is_match(url) {
                    routes::create_cycles::handler(request, map);
                } else {
                    let response = Response::from_string("invalid request").with_status_code(400);
                    request.respond(response).ok();
                }

                info!("request: {} | elapsed: {} secs", req_url, now.elapsed().as_secs_f32());
            }
        }));
    }

    for h in handlers {
        h.join().unwrap();
    }
}