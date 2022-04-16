use rand::Rng;
use rand;

use crate::state::{GeneExpression, Evolver};
use crate::state::models::{Traits, Constants};

impl Traits {

    pub fn new(constants: &Constants) -> Traits {
        let mut range = rand::thread_rng();
        return Traits {
            restitution: 0.3,//0.3,
            friction: 0.1,//0.2,
            stamina: range.gen_range(0f32, 100f32),//100.0,
            block_mass: range.gen_range(5.0f32, 10.0f32),
            block_amount: constants.initial_block_amount,
            block_size: constants.initial_block_size,
            color: vec![0.0, 0.0, 0.0, 1.0],
            strength: 100.0,
            gene_codes: Vec::new(),
        };
    }

    pub fn get_net_speed(&self) -> f32 {
        let net_mass =  self.get_net_mass();
        let max_mass = self.block_amount as f32 * 10.0 * 2.0;
        return (1.0 / net_mass) * max_mass * 10000.0;
    }

    pub fn get_net_mass(&self) -> f32 {
        return self.block_mass * self.block_amount as f32;
    }   

    pub fn get_stamina_factor(&self) -> f32 {
        return self.block_mass / 3.0;
    }

}

impl Evolver for Traits {
    fn evolve(&self, constants: &Constants) -> Traits {
        let mut new_traits = self.clone();
        let mut range = rand::thread_rng();

        let size_chance = range.gen_range(0.0, 1.0);
        if size_chance >= constants.block_size_evolve_chance {
            let pm = range.gen_range(0.0, 1.0);
            let nudge = range.gen_range(constants.min_block_size_nudge, constants.max_block_size_nudge);

            if pm >= 0.5 {
                new_traits.block_size += nudge;
            } else {
                new_traits.block_size -= nudge;
            }            
        }

        return new_traits;
    }
}

impl GeneExpression for Traits {
    fn gene_codes(&self, constants: &Constants) -> Vec<u8> {
        return Vec::new();
    }
}