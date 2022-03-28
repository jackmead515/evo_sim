use rand::Rng;
use rand;

use crate::state::{GeneExpression, Evolver};
use crate::state::models::{Brain, Traits, Bounds, Creature};
use crate::state::models::{Point, Constants};

impl Creature {
    pub fn new(creature_id: u32, constants: &Constants) -> Creature {
        return Creature {
            creature_id: creature_id,
            brain: Brain::new(constants),
            traits: Traits::new(constants),
            bounds: Bounds::new(constants),
        }
    }
}

impl Evolver for Creature {
    fn evolve(&self, constants: &Constants) -> Creature {
        let mut new_creature = self.clone();

        new_creature.brain = new_creature.brain.evolve(constants);
        new_creature.traits = new_creature.traits.evolve(constants);
        new_creature.bounds = new_creature.bounds.evolve(constants);

        return new_creature;
    }
}

impl GeneExpression for Creature {
    fn gene_codes(&self, constants: &Constants) -> Vec<u8> {
        let mut norms: Vec<u8> = Vec::new();
        norms.extend(self.brain.gene_codes(constants));
        norms.extend(self.traits.gene_codes(constants));
        norms.extend(self.bounds.gene_codes(constants));
        return norms;
    }
}

