use rand::Rng;
use rand;

use crate::state::models::{Brain, Neuron, Activation};

impl Neuron {

    pub fn random(id: u8, weight_size: u8) -> Self {
        let mut weights = Vec::new();
        let mut range = rand::thread_rng();

        for _ in 0..weight_size {
            weights.push(range.gen_range(0.0f32, 1.0f32));
        }

        return Neuron {
            activation: Activation::Sigmoid,
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
            Activation::Sigmoid => { 1.0 / (1.0 + (-total).exp()) },
            Activation::Softmax => {
                panic!("cannot use softmax");
            }
        };

        return total;
    }

}

impl Brain {

    pub fn new_random(id: usize, brain_size: usize, input_size: usize, output_size: usize) -> Self {
        let mut brain = Brain {
            creature_id: id,
            hidden: Vec::with_capacity(brain_size as usize),
            output: Vec::with_capacity(output_size as usize),
            hidden_buffer: Vec::with_capacity(brain_size as usize),
            output_buffer: Vec::with_capacity(output_size as usize),
            activation: Activation::Softmax,
        };

        for id in 0..brain_size {
            brain.hidden.push(Neuron::random(id as u8, input_size as u8));
            brain.hidden_buffer.push(0.0);
        }

        for id in 0..output_size {
            brain.output.push(Neuron::random(id as u8, brain_size as u8));
            brain.output_buffer.push(0.0);
        }

        return brain;
    }
    
    pub fn compute(&mut self, inputs: &Vec<f32>) -> (Vec<f32>, u8) {
        let hidden_size = self.hidden.len();
        let output_size = self.output.len();

        // compute inputs on hidden layer
        for i in 0..hidden_size {
            self.hidden_buffer[i] = self.hidden[i].compute(inputs);
        }

        // compute output of hidden layer on output layer
        for i in 0..output_size {
            self.output_buffer[i] = self.output[i].compute(&self.hidden_buffer);
        }

        // Apply activation function on output layer
        return match self.activation {
            Activation::Softmax => {
                let mut exps = Vec::with_capacity(output_size);
                for output in self.output_buffer.iter() {
                    exps.push(output.exp());
                }

                let sum: f32 = exps.iter().sum();
                let mut max = 0.0;
                let mut max_index: u8 = 0;
                let mut outputs = Vec::with_capacity(output_size);
                for i in 0..self.output_buffer.len() {
                    let output = exps[i] / sum;
                    if output > max {
                        max = output;
                        max_index = i as u8;
                    }
                    outputs.push(output);
                }
                return (outputs, max_index);
            },
            Activation::Sigmoid => {
                panic!("cannot use sigmoid");
            },
            _ => (Vec::new(), 0),
        };
    }

}