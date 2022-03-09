use std::thread::{JoinHandle};
use std::sync::{Arc, Mutex};
use std::sync::mpsc::channel;
use std::collections::HashMap;
use std::thread;

use rand::Rng;
use rand::prelude::*;
use rand;


use crate::state::models::{Brain, Neuron, Activation, Creature};
use crate::state::simulation::Constants;

fn normalize(inputs: &mut Vec<f32>) {
    let mut min: f32 = f32::MAX;
    let mut max: f32 = f32::MIN;

    for i in inputs.iter() {
        if i < &min {
            min = *i;
        }
        if i > &max {
            max = *i;
        }
    }

    for i in inputs.iter_mut() {
        *i = (*i - min) / (max - min);
    }
}

impl Neuron {

    pub fn random(weight_size: u8) -> Self {
        let mut weights: Vec<f32> = Vec::new();
        let mut range = rand::thread_rng();

        for _ in 0..weight_size {
            weights.push(range.gen());
        }

        return Neuron {
            activation: 1,
            weights: weights,
            bias: range.gen(),
        }
    }

    /// Computes the sigmoid output total given the
    /// input vector that is equal in length to
    /// the weights of this neuron.
    pub fn compute(&self, inputs: &Vec<f32>) -> f32 {
        let mut total = 0.0;

        for index in 0..self.weights.len() {
            let weight = self.weights[index];
            let input = inputs[index];
            total += weight * input;
        }

        total = match self.activation {
            1 => {
                1.0 / (1.0 + (-total).exp())
            },
            _ => {
                panic!("cannot use softmax");
            }
        };

        return total;
    }

}

impl Brain {

    pub fn new(constants: &Constants) -> Self {
        let mut brain = Brain {
            hidden: Vec::with_capacity(constants.brain_size as usize),
            output: Vec::with_capacity(constants.output_size as usize),
            activation: 2,
        };

        for _ in 0..constants.brain_size {
            brain.hidden.push(Neuron::random(constants.input_size as u8));
        }

        for _ in 0..constants.output_size {
            brain.output.push(Neuron::random(constants.brain_size as u8));
        }

        return brain;
    }
    
    pub fn compute(&self, inputs: &mut Vec<f32>) -> (Vec<f32>, u8) {
        let hidden_size = self.hidden.len();
        let output_size = self.output.len();

        // TODO: would be nice to allocate this on the stack in the future
        // but I serialize the entire brain so I need a lazy static
        // somewhere to act as a hashmap for the buffers
        let mut hidden_buffer = Vec::with_capacity(hidden_size);
        let mut output_buffer = Vec::with_capacity(output_size);

        // normalize inputs from 0 -> 1
        normalize(inputs);

        // compute inputs on hidden layer
        for i in 0..hidden_size {
            hidden_buffer.push(self.hidden[i].compute(inputs));
        }

        // compute output of hidden layer on output layer
        for i in 0..output_size {
            output_buffer.push(self.output[i].compute(&hidden_buffer));
        }

        // Apply activation function on output layer
        return match self.activation {
            2 => {
                let mut exps = Vec::with_capacity(output_size);
                let mut exp_sum = 0.0;
                for output in output_buffer.iter() {
                    let exp = output.exp();
                    exp_sum += exp;
                    exps.push(exp);
                }

                let mut max = 0.0;
                let mut max_index: u8 = 0;
                let mut outputs = Vec::with_capacity(output_size);
                for i in 0..output_buffer.len() {
                    let output = exps[i] / exp_sum;
                    if output > max {
                        max = output;
                        max_index = i as u8;
                    }
                    outputs.push(output);
                }
                return (outputs, max_index);
            },
            1 => {
                panic!("cannot use sigmoid");
            },
            _ => (Vec::new(), 0),
        };
    }

    pub fn evolve(&self, constants: &Constants) -> Brain {
        let new_brain = self.clone();

        return new_brain;
    } 

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_have_no_decisions() {
        let constants = Constants {
            world_width: 800,
            world_height: 640,
            max_cycles: 1000,
            max_steps: 1000,
            creature_amount: 100,
            brain_size: 5,
            input_size: 5,
            output_size: 5,
            block_amount: 10,
            block_size: 5.0
        };

        let brain = Brain::new(&constants);

        let mut inputs = vec![0.0, 0.0, 0.0, 0.0, 0.0];

        let (outputs, decision) = brain.compute(&mut inputs);

        println!("outputs: {:?}, decision: {}", outputs, decision);

        assert_eq!(1, 1);
    }
}

// pub fn compute(
//     creature_map: Arc<Mutex<HashMap<u32, Creature>>>,
//     decision_map: Arc<Mutex<HashMap<u32, u8>>>,
// ) -> JoinHandle<()> {
//     let creature_map = creature_map.clone();
//     let decision_map = decision_map.clone();
//     return thread::spawn(move || {

//         let (sender, receiver) = channel();

//         for _ in 0..8 {
//             let decision_map = decision_map.clone();
//             let handle = thread::spawn(move || {
//                 let (_outputs, decision) = ccreature.brain.compute(&vec![0.1, 0.2, 0.3, 0.4, 0.5]);
                
//                 match decision_map.lock() {
//                     Ok(mut map) => {
//                         map.insert(creature_id, decision);
//                     },
//                     Err(_) => {},
//                 };
//             });
//             handles.push(handle);
//         }

//         match creature_map.lock() {
//             Ok(map) => {
//                 for (creature_id, creature) in map.iter() {
                    
//                 }
//             }
//         }

        
//     }); 
// }