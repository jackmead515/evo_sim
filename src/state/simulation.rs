use std::collections::HashMap;

use rand::Rng;
use rand::prelude::ThreadRng;
use rand;
use rapier2d::prelude::*;

use crate::state::Evolver;
use crate::state::models::{Cycle, Step, Creature};

pub struct Constants {
    pub world_width: u32,
    pub world_height: u32,
    pub max_cycles: u32,
    pub max_steps: u32,
    pub creature_amount: u32,
    pub brain_size: u32,
    pub input_size: u32,
    pub output_size: u32,
    pub block_amount: u32,
    pub block_size: f32,
}

pub struct Simulation {
    pub simulation_id: u32,
    pub constants: Constants,
    pub computed_cycles: u32,
    pub current_cycle: Option<Cycle>,
}

impl Simulation {

    pub fn new(simulation_id: u32) -> Simulation {
        return Simulation {
            simulation_id: simulation_id,
            computed_cycles: 0,
            current_cycle: None,
            constants: Constants {
                world_width: 800,
                world_height: 640,
                max_cycles: 1000,
                max_steps: 1000,
                creature_amount: 20,
                brain_size: 50,
                input_size: 5,
                output_size: 5,
                block_amount: 10,
                block_size: 5.0,
            }
        };
    }

    pub fn next_cycle(&mut self) -> Option<Cycle> {
        if self.computed_cycles >= self.constants.max_cycles {
            return None;
        }

        match &self.current_cycle {
            Some(cycle) => {
                return Some(cycle.evolve(&self.constants));
            },
            None => {
                return Some(Cycle::new(&self.constants));
            }
        };
    }

}

impl Cycle {

    pub fn new(constants: &Constants) -> Cycle {
        let mut cycle = Cycle {
            cycle_id: 0,
            creatures: HashMap::new(),
            walls: Vec::new(),
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
            dynamic_walls: Vec::new()
        };
    }

}