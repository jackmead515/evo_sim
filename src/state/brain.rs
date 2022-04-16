use std::thread::{JoinHandle};
use std::sync::{Arc, Mutex};
use std::sync::mpsc::channel;
use std::collections::HashMap;
use std::thread;
use std::str;

use rand::Rng;
use rand::prelude::*;
use rand;

use crate::state::models::*;

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

    // prevent divide by zero affect.
    // can only happen if all inputs are 0!
    // max and min will never be negative
    // either
    let bottom = max - min;
    if bottom == 0.0 {
        return;
    }

    for i in inputs.iter_mut() {
        *i = (*i - min) / bottom;
    }
}

impl Neuron {

    pub fn random(weight_size: u32) -> Self {
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

        total += self.bias;

        total = match self.activation {
            1 => {
                // sigmoid activation
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
            hidden: Vec::with_capacity(constants.initial_brain_size as usize),
            output: Vec::with_capacity(constants.brain_output_size as usize),
            activation: 2,
        };

        for _ in 0..constants.initial_brain_size {
            brain.hidden.push(Neuron::random(constants.brain_input_size));
        }

        for _ in 0..constants.brain_output_size {
            brain.output.push(Neuron::random(constants.initial_brain_size));
        }

        return brain;
    }

    /// Generates a set of gene codes that describe the brain 
    /// and it's weights. The algorithm splits the brain weights 
    /// into partitions, totally each up and normalizing the values
    /// to be between 65 and 90 (A to Z in the ASCII table).
    pub fn gene_codes(&self, constants: &Constants) -> Vec<u8> {
        let mut weight_norms: Vec<u8> = Vec::new();

        // 65 - 90 == A - Z
        let min_char = 65.0;
        let max_char = 90.0;

        // total size of the hidden weights. Plus one for the bias.
        let max_hidden_size: f32 = (constants.brain_input_size + 1) as f32;
        let max_output_size: f32 = (self.hidden.len() + 1) as f32;
        
        // sum up the weights and bias of the hidden layer
        for neuron in self.hidden.iter() {
            let weight_sum = neuron.weights.iter().sum::<f32>() + neuron.bias;
            let norm = ((weight_sum / max_hidden_size) * (max_char - min_char)) + min_char;
           
            // should be a safe cast norm to u8 because 
            // ascii is between 65 - 90. u8 is between 0 and 255.
            weight_norms.push(norm as u8);
        }

        // sum up the weights and bias of the output layer
        for neuron in self.output.iter() {
            let weight_sum = neuron.weights.iter().sum::<f32>() + neuron.bias;
            let norm = ((weight_sum / max_output_size) * (max_char - min_char)) + min_char;
            weight_norms.push(norm as u8);
        }

        return weight_norms;
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
                // softmax activation
                let mut exps = Vec::with_capacity(output_size);
                let mut exp_sum = 0.0;

                // compute the natural exponential
                // and the sum of the exponentials
                // at the same time
                for output in output_buffer.iter() {
                    let exp = output.exp();
                    exp_sum += exp;
                    exps.push(exp);
                }

                let mut max = 0.0;
                let mut max_index: u8 = 0;
                let mut outputs = Vec::with_capacity(output_size);
                for i in 0..output_buffer.len() {
                    // output of softmax is = exp / sum(exp)
                    let output = exps[i] / exp_sum;

                    // max decision based on max value
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
        let mut new_brain = self.clone();
        let mut range = rand::thread_rng();

        // if, randomly, we evolve the brain, do it!
        let chance = range.gen_range(0.0f32, 1.0f32);
        if chance >= constants.brain_evolve_chance {

            let hidden_wl = new_brain.hidden[0].weights.len();
            let output_wl = new_brain.output[0].weights.len();

            // for each hidden neuron, nudge a random weight
            for neuron in 0..new_brain.hidden.len() {
                let weight = range.gen_range(0, hidden_wl-1);
                let nudge = range.gen_range(constants.min_brain_weight_nudge, constants.max_brain_weight_nudge);
                if nudge + new_brain.hidden[neuron].weights[weight] > 1.0 {
                    new_brain.hidden[neuron].weights[weight] -= nudge;
                } else {
                    new_brain.hidden[neuron].weights[weight] += nudge;
                }
            }

            // for each outout neuron, nudge a random weight
            for neuron in 0..new_brain.output.len() {
                let weight = range.gen_range(0, output_wl-1);
                let nudge = range.gen_range(constants.min_brain_weight_nudge, constants.max_brain_weight_nudge);
                if nudge + new_brain.output[neuron].weights[weight] > 1.0 {
                    new_brain.output[neuron].weights[weight] -= nudge;
                } else {
                    new_brain.output[neuron].weights[weight] += nudge;
                }
            }
        }

        return new_brain;
    } 

}