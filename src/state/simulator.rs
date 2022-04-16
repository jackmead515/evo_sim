use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex, RwLock};
use std::sync::atomic::AtomicBool;
use std::fs;
use std::thread;
use std::io::prelude::*;
use std::sync::mpsc::channel;
use std::time::Duration;
use std::ffi::OsStr;

use rand::Rng;
use notify::{RecommendedWatcher, Watcher, RecursiveMode, DebouncedEvent};
use rand::prelude::ThreadRng;
use rapier2d::prelude::*;
use rand;
use bytes::Buf;
use prost::Message;
use log::{info};
use flate2::write::ZlibDecoder;


use crate::state::Evolver;
use crate::state::models::{Cycle, Step, Creature, Simulation, Constants};

pub struct SimulationMap {
    map: RwLock<HashMap<u32, Simulator>>,
    watch_thread_handle: Option<thread::JoinHandle<()>>
}

pub struct Simulator {
    pub simulation: Simulation,
    pub current_cycle: Option<Cycle> 
}

impl SimulationMap {
    
    pub fn new() -> Self {
        SimulationMap {
            map: RwLock::new(HashMap::new()),
            watch_thread_handle: None
        }
    }

    pub fn get_pop(&self, simulation_id: &u32) -> Option<Simulator> {
        if let Ok(mut map) = self.map.write() {
            return map.remove(simulation_id);
        }

        return None;
    }

    pub fn insert(&self, simulation_id: u32, simulation: Simulator) {
        if let Ok(mut map) = self.map.write() {
            map.insert(simulation_id, simulation);
        }
    }

    pub fn contains(&self, simulation_id: &u32) -> bool {
        if let Ok(map) = self.map.read() {
            return map.contains_key(&simulation_id);
        }

        return false;
    }

    pub fn watch_changes(&mut self) {
        let join_handle = thread::spawn(move || {
            let (tx, rx) = channel();
            let mut watcher: RecommendedWatcher = Watcher::new(tx, Duration::from_secs(1)).unwrap();
            watcher.watch("./simulations", RecursiveMode::Recursive).unwrap();

            loop {
                match rx.recv() {
                    Ok(event) => {
                        match event {
                            DebouncedEvent::Create(path) => {
                                let file_name = path.file_name().unwrap().to_str().unwrap();
                                println!("watch file created: {:?}", file_name);
                            },
                            DebouncedEvent::Remove(path) => {

                            },
                            DebouncedEvent::Write(path) => {

                            },
                            _ => {}
                        }
                    },
                    Err(err) => {
                        println!("watch error: {:?}", err);
                    }
                }
            }
        });

        self.watch_thread_handle = Some(join_handle);
    }

    pub fn sync_from_disk(&self) {
        if let Ok(mut map) = self.map.write() {
            let sims_folder = "./simulations";
            fs::create_dir_all(&sims_folder).unwrap();

            let folders = fs::read_dir(sims_folder).unwrap();

            for folder in folders {
                let folder_name = folder.unwrap().file_name().into_string().unwrap();
                let sim_id = folder_name.split("_").nth(1).unwrap().parse::<u32>().unwrap();
                let sim_file = format!("{}/{}/simulation.zip", sims_folder, folder_name);
                let cycles_folder = format!("{}/{}/cycles", sims_folder, folder_name);

                // decompress and load the simulation
                let serialized = fs::read(sim_file).unwrap();
                let mut decompressor = ZlibDecoder::new(Vec::new());
                decompressor.write_all(&serialized).unwrap();
                let buffer = decompressor.finish().unwrap();
                let simulation = Simulation::decode(&buffer[..]).unwrap();

                let mut simulator = Simulator::new(sim_id);
                simulator.simulation = simulation;

                // create the folder for the simulation
                fs::create_dir_all(&cycles_folder).unwrap();

                let cycles = fs::read_dir(cycles_folder).unwrap();

                let mut cycle_ids: Vec<u32> = cycles
                    .map(|p| {
                        let file_name = p.unwrap().file_name().into_string().unwrap();
                        let prefix = file_name.split(".zip").nth(0).unwrap();
                        return prefix.split("_").nth(1).unwrap().parse::<u32>().unwrap();
                    })
                    .collect();
                
                cycle_ids.sort();

                info!("simulation {} with cycles: {:?}", sim_id, cycle_ids);

                if cycle_ids.len() > 0 {
                    match cycle_ids.get(cycle_ids.len()-1) {
                        Some(cycle_id) => {
                            let cycle_file = format!("{}/{}/cycles/cycle_{}.zip", sims_folder, folder_name, cycle_id);
                            let serialized = fs::read(cycle_file).unwrap();
                            let mut decompressor = ZlibDecoder::new(Vec::new());
                            decompressor.write_all(&serialized).unwrap();
                            let buffer = decompressor.finish().unwrap();
                            let cycle = Cycle::decode(&buffer[..]).unwrap();
                            simulator.current_cycle = Some(cycle);
                        },
                        None => {}
                    };
                }

                map.insert(sim_id, simulator);
            }
        }
    }
}

impl Simulator {

    pub fn new(simulation_id: u32) -> Simulator {
        return Simulator {
            simulation: Simulation {
                simulation_id: simulation_id,
                cycle_ids: Vec::new(),
                constants: Constants::new()
            },
            current_cycle: None
        };
    }

    pub fn next_cycle(&self) -> Cycle {
        match &self.current_cycle {
            Some(cycle) => {
                return cycle.evolve(&self.simulation.constants);
            },
            None => {
                return Cycle::new(&self.simulation.constants);
            }
        };
    }

}