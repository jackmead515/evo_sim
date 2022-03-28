use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex, RwLock};
use std::sync::atomic::AtomicBool;
use std::fs;

use rand::Rng;
use rand::prelude::ThreadRng;
use rapier2d::prelude::*;
use rand;
use bytes::Buf;
use prost::Message;

use crate::state::Evolver;
use crate::state::models::{Cycle, Step, Creature, Simulation, Constants};

pub struct SimulationMap {
    map: RwLock<HashMap<u32, Simulator>>
}

impl SimulationMap {
    
    pub fn new() -> Self {
        SimulationMap {
            map: RwLock::new(HashMap::new())
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

    pub fn sync_from_disk(&self) {
        if let Ok(mut map) = self.map.write() {
            let sims_folder = "./simulations";

            let folders = fs::read_dir(sims_folder).unwrap();

            for folder in folders {
                let folder_name = folder.unwrap().file_name().into_string().unwrap();
                let sim_id = folder_name.split("_").nth(1).unwrap().parse::<u32>();
                let sim_file = format!("{}/{}/simulation.zip", sims_folder, folder_name);

                // let buffer = fs::read(sim_file).unwrap();
                // buffer = bytes::Buf::copy_from_slice(&buffer);

                // let simulation = Simulation::decode(buffer).unwrap();

                // simulator

                // map.insert(sim_id, simulation).unwrap();
            }

        }
    }

    // pub fn get(&self, simulation_id: &u32) -> Option<&Simulation> {
    //     if let Ok(map) = self.map.read() {
    //         return map.get(simulation_id);
    //     }

    //     return None;
    // }

    // pub fn get_mut(&self, simulation_id: &u32) -> Option<&mut Simulation> {
    //     if let Ok(mut map) = self.map.write() {
    //         return map.get_mut(simulation_id);
    //     }

    //     return None;
    // }

}

impl Constants {
    pub fn new() -> Constants {
        return Constants {
            world_width: 2000,
            world_height: 2000,
            max_steps: 1000,
            creature_amount: 20,
            initial_brain_size: 50,
            max_brain_size: 100,
            min_brain_size: 10,
            brain_input_size: 5,
            brain_output_size: 5,
            initial_block_amount: 5,
            min_block_amount: 2,
            max_block_amount: 20,
            initial_block_size: 5.0,
            max_block_size: 10.0,
            min_block_size: 3.0
        }
    }
}

pub struct Simulator {
    pub simulation: Simulation,
    pub current_cycle: Option<Cycle> 
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

impl Cycle {

    pub fn new(constants: &Constants) -> Cycle {
        let mut cycle = Cycle {
            cycle_id: 0,
            creatures: HashMap::new(),
            steps: Vec::new()
        };

        for creature_id in 0..constants.creature_amount {
            let creature = Creature::new(creature_id as u32, &constants);
            cycle.creatures.insert(creature_id, creature);
        }

        return cycle;
    }

    pub fn evolve(&self, constants: &Constants) -> Cycle {
        let mut new_cycle = self.clone();

        new_cycle.cycle_id += 1;
        new_cycle.steps = Vec::new();

        for (creature_id, creature) in new_cycle.creatures.iter_mut() {
            *creature = creature.evolve(constants);
        }

        return new_cycle;
    }

    pub fn next_step(&self, constants: &Constants) -> Option<Step> {
        if self.steps.len() >= constants.max_steps as usize {
            return None;
        }

        if self.steps.len() >= 1 {
            let mut new_step = self.steps[self.steps.len()-1].clone();
            new_step.step_id = self.steps.len() as u32;
            return Some(new_step);
        }
        
        return Some(Step::new());
    }

}

impl Step {

    pub fn new() -> Step {
        return Step {
            step_id: 0,
            states: HashMap::new(),
            boundaries: Vec::new()
        };
    }

}