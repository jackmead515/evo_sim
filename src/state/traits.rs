use rand::Rng;
use rand;

use crate::state::models::{Traits, Constants};

impl Traits {

    pub fn new(constants: &Constants) -> Traits {
        return Traits {
            restitution: 0.3
        };
    }

    pub fn evolve(&self, constants: &Constants) -> Traits {
        let new_traits = self.clone();

        return new_traits;
    } 

}