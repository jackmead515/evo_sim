use rand::Rng;
use rand;

use crate::state::{GeneExpression, Evolver};
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
    fn gene_codes(&self, constants: &Constants) -> Vec<String> {
        let mut genes: Vec<String> = Vec::new();

        genes.extend(self.brain.gene_codes(constants));
        genes.extend(self.traits.gene_codes(constants));
        genes.extend(self.bounds.gene_codes(constants));

        return genes;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_generate_some_genes() {
        let constants = Constants {
            world_width: 800,
            world_height: 640,
            max_cycles: 1000,
            max_steps: 1000,
            creature_amount: 100,
            brain_size: 50,
            input_size: 5,
            output_size: 5,
            block_amount: 10,
            block_size: 5.0
        };

        let creature = Creature::new(0, &constants);

        let gene_codes = creature.gene_codes(&constants);

        println!("{:?}", gene_codes);
    }
}

