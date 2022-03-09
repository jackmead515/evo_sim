use rand::Rng;
use rand;

use crate::state::models::{Traits};
use crate::state::simulation::Constants;

impl Traits {

    pub fn new(constants: &Constants) -> Traits {
        return Traits {
            restitution: 0.3,
            friction: 0.2,
            stamina: 1000.0,
            block_mass: 5.0,
            block_amount: constants.block_amount,
            strength: 100.0,
        };
    }

    pub fn evolve(&self, constants: &Constants) -> Traits {
        let new_traits = self.clone();

        return new_traits;
    }

    pub fn get_net_speed(&self) -> f32 {
        let net_speed = self.strength - self.get_net_mass();
        if net_speed > 0.0 {
            return net_speed;
        }

        return net_speed;
    }

    pub fn get_net_mass(&self) -> f32 {
        return self.block_mass * self.block_amount as f32;
    }   

    pub fn get_stamina_factor(&self) -> f32 {
        return self.block_mass / 3.0;
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

        assert_eq!(50.0, traits.get_net_mass());
        assert_eq!(50.0, traits.get_net_speed());
        assert_eq!(1.6666666, traits.get_stamina_factor());

    }
}