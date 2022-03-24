use rand::Rng;
use rand;

use crate::state::{GeneExpression, Evolver};
use crate::state::models::{Traits};
use crate::state::simulation::Constants;

impl Traits {

    pub fn new(constants: &Constants) -> Traits {
        let mut range = rand::thread_rng();
        return Traits {
            restitution: 0.3,//0.3,
            friction: 0.1,//0.2,
            stamina: range.gen_range(0f32, 100f32),//100.0,
            block_mass: range.gen_range(5.0f32, 10.0f32),
            block_amount: constants.block_amount,
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
        let new_traits = self.clone();

        return new_traits;
    }
}

impl GeneExpression for Traits {
    fn gene_codes(&self, constants: &Constants) -> Vec<u8> {
        return Vec::new();
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_compute_traits() {
        let constants = Constants {
            world_width: 800,
            world_height: 640,
            max_cycles: 1000,
            max_steps: 1000,
            creature_amount: 100,
            brain_size: 50,
            input_size: 4,
            output_size: 5,
            block_amount: 10,
            block_size: 5.0
        };
        
        let mut traits = Traits::new(&constants);

        println!("traits: {:?}", traits);
        println!("net mass: {}", traits.get_net_mass());
        println!("net speed: {}", traits.get_net_speed());
        println!("stamina factor: {}", traits.get_stamina_factor());

        assert_eq!(1, 1);

    }
}