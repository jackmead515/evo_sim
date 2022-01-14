use rand::Rng;
use rand;

use crate::state::models::{Brain, Traits, Bounds, Creature};
use crate::state::simulation::Constants;

impl Creature {


    pub fn new(creature_id: u32, constants: &Constants) -> Creature {
        return Creature {
            creature_id: creature_id,
            brain: Brain::new(constants),
            traits: Traits::new(constants),
            bounds: Bounds::new(constants),
        }
    }

    pub fn evolve(&self, constants: &Constants) -> Creature {
        let mut new_creature = self.clone();

        new_creature.brain = new_creature.brain.evolve(constants);
        new_creature.traits = new_creature.traits.evolve(constants);
        new_creature.bounds = new_creature.bounds.evolve(constants);

        return new_creature;
    }

}