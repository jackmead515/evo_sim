use std::collections::HashMap;

use rand::Rng;
use rand::prelude::ThreadRng;
use rand;
use rapier2d::prelude::*;


use crate::state::create;
use crate::state::models::{Simulation, Cycle, Step, Brain, Traits};

impl Simulation {

    pub fn next_cycle(&mut self) -> Cycle {
        if self.cycles.len() > 0 {
            let last_cycle = &self.cycles[self.cycles.len() - 1];
        }

        let mut cycle = Cycle {
            cycle_id: 0,
            brain_map: HashMap::new(),
            trait_map: HashMap::new(),
            walls: Vec::new(),
            steps: Vec::new()
        };

        for id in 0..self.constants.creature_amount - 1 {
            let brain = Brain::new_random(id, self.constants.brain_size, self.constants.input_size, self.constants.output_size);
            let traits = Traits {
                color: [255, 0, 0, 1],
                creature_id: id
            };

            cycle.brain_map.insert(id, brain);
            cycle.trait_map.insert(id, traits);
        }

        return cycle;
    }

    pub fn next_step(&mut self, id: usize) -> (Step, Vec<(RigidBody, Collider)>) {
        let mut step = Step {
            step_id: id,
            creatures: Vec::new(),
            dynamic_walls: Vec::new()
        };

        let mut bodies: Vec<(RigidBody, Collider)> = Vec::new();

        for id in 0..self.constants.creature_amount - 1 {
            let (creature, body, collider) = create::creature(id, self.constants.block_amount, self.constants.block_size);
            step.creatures.push(creature);
            bodies.push((body, collider));
        }

        return (step, bodies);
    }

    pub fn last_cycle(&mut self) -> &mut Cycle {
        let index = self.cycles.len() - 1;
        return &mut self.cycles[index];
    }

}