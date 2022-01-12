use rand::Rng;
use rand;

use crate::state::models::{Brain, Neuron, Activation, Constants};

impl Neuron {

    pub fn random(weight_size: u8) -> Self {
        let mut weights = Vec::new();
        let mut range = rand::thread_rng();

        for _ in 0..weight_size {
            weights.push(range.gen_range(0.0f32, 1.0f32));
        }

        return Neuron {
            activation: 1,
            weights: weights,
            bias: range.gen_range(0.0f32, 1.0f32),
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
            1 => { 1.0 / (1.0 + (-total).exp()) },
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

        return brain;
    }
    
    pub fn compute(&mut self, inputs: &Vec<f32>) -> (Vec<f32>, u8) {
        let hidden_size = self.hidden.len();
        let output_size = self.output.len();

        let mut hidden_buffer = Vec::with_capacity(hidden_size);
        let mut output_buffer = Vec::with_capacity(output_size);

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
                for output in output_buffer.iter() {
                    exps.push(output.exp());
                }

                let sum: f32 = exps.iter().sum();
                let mut max = 0.0;
                let mut max_index: u8 = 0;
                let mut outputs = Vec::with_capacity(output_size);
                for i in 0..output_buffer.len() {
                    let output = exps[i] / sum;
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