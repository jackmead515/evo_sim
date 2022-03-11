pub mod models;
pub mod bounds;
pub mod brain;
pub mod traits;
pub mod creature;
pub mod simulation;

use crate::state::simulation::Constants;

pub trait GeneExpression {
    fn gene_codes(&self, constants: &Constants) -> Vec<String>;
}

pub trait Evolver {
    fn evolve(&self, constants: &Constants) -> Self;
}