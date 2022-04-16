use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex, RwLock};
use std::sync::atomic::AtomicBool;
use std::fs;
use std::thread;
use std::io::prelude::*;
use std::sync::mpsc::channel;
use std::time::Duration;

use rand::Rng;
use notify::{RecommendedWatcher, Watcher, RecursiveMode};
use rand::prelude::ThreadRng;
use rapier2d::prelude::*;
use rand;
use bytes::Buf;
use prost::Message;
use log::{info};
use flate2::write::ZlibDecoder;


use crate::state::Evolver;
use crate::state::models::{Cycle, Step, Creature, Simulation, Constants};

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
            min_brain_weight_nudge: 0.01,
            max_brain_weight_nudge: 0.5,

            brain_evolve_chance: 0.5,
            block_amount_evolve_chance: 0.05,
            block_arrange_evolve_chance: 0.1,
            block_size_evolve_chance: 0.1,

            initial_block_amount: 5,
            min_block_amount: 2,
            max_block_amount: 20,
            initial_block_size: 5.0,
            max_block_size: 10.0,
            min_block_size: 3.0,
            min_block_size_nudge: 0.01,
            max_block_size_nudge: 0.9,   
        }
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