use std::collections::HashMap;

use rand::Rng;
use rand::prelude::ThreadRng;
use rand;
use rapier2d::prelude::*;

use crate::state::models::{Simulation, Cycle, Step, Constants, Creature};

impl Simulation {

    pub fn new(simulation_id: u32) -> Simulation {
        return Simulation {
            simulation_id: simulation_id,
            cycles: Vec::new(),   
            constants: Constants {
                max_cycles: 1000,
                max_steps: 1000,
                creature_amount: 10,
                brain_size: 50,
                input_size: 4,
                output_size: 5,
                block_amount: 10,
                block_size: 5.0
            }
        };
    }

    pub fn next_cycle(&mut self) -> Option<Cycle> {
        if self.cycles.len() >= self.constants.max_cycles as usize {
            return None;
        }

        if self.cycles.len() >= 1 {
            return Some(self.cycles[self.cycles.len()-1].evolve(&self.constants));
        }

        return Some(Cycle::new(&self.constants));
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