use rand::Rng;
use rand;

use crate::state::models::{Traits};
use crate::state::simulation::Constants;

impl Traits {

    pub fn new(constants: &Constants) -> Traits {
        return Traits {
            restitution: 0.3,
            friction: 0.2,
            stamina: 100.0,
            mass: 0.0,
            strength: 5.0,
        };
    }

    pub fn evolve(&self, constants: &Constants) -> Traits {
        let new_traits = self.clone();

        return new_traits;
    }

    pub fn get_net_speed(&self) -> f32 {
        let net_speed = self.strength - self.mass.powf(2.0);
        if net_speed > 0.0 {
            return net_speed;
        }

        return net_speed;
    }

    pub fn get_stamina_factor(&self) -> f32 {
        return self.mass / 3.0;
    }

}