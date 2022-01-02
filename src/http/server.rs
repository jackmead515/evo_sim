
use tiny_http::{Response, Server, Method};
use std::time::{Instant};
use log::{info};

use crate::engine;

pub fn start() {
    info!("Starting evo sim server");

    let server = Server::http("0.0.0.0:8000").unwrap();

    for mut request in server.incoming_requests() {
        let method = request.method();
        let url = request.url();

        info!("Request: {} {}", method, url);

        if matches!(method, Method::Post) && url == "/perform-cycle" {
            info!("Request to perform a cycle recieved");
            
            let now = Instant::now();
            engine::perform_cycle();
            info!("{} ms", now.elapsed().as_millis());

            let response = Response::from_string("performed cycle").with_status_code(200);
            request.respond(response).ok();

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