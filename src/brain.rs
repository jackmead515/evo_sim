use rand::Rng;
use rand;

#[derive(Clone, Debug)]
pub enum Activation {
    Sigmoid,
    Softmax
}

#[derive(Clone, Debug)]
pub struct Brain {
    pub hidden: Vec<Neuron>,
    pub output: Vec<Neuron>,
    pub activation: Activation,
}

#[derive(Clone, Debug)]
pub struct Neuron {
    pub id: u8,
    pub bias: f32,
    pub activation: Activation,
    pub weights: Vec<f32>,
}

impl Neuron {

    pub fn random(id: u8, weight_size: u8) -> Self {
        let mut weights = Vec::new();
        let mut range = rand::thread_rng();

        for _ in 0..weight_size {
            weights.push(range.gen_range(0.0f32, 1.0f32));
        }

        return Neuron {
            id: id,
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

    pub fn new_random(brain_size: u8, input_size: u8, output_size: u8) -> Self {
        let mut brain = Brain {
            hidden: Vec::new(),
            output: Vec::new(),
            activation: Activation::Softmax,
        };

        for id in 0..brain_size {
            brain.hidden.push(Neuron::random(id, input_size));
        }

        for id in 0..output_size {
            brain.output.push(Neuron::random(id, brain_size));
        }

        return brain;
    }

    pub fn compute(&mut self, inputs: &Vec<f32>) -> (Vec<f32>, u8) {
        let hidden_size = self.hidden.len();
        let output_size = self.output.len();

        // compute inputs on hidden layer
        let mut hidden: Vec<f32> = Vec::with_capacity(hidden_size);
        for i in 0..hidden_size {
            hidden.push(self.hidden[i].compute(inputs));
        }

        // compute output of hidden layer on output layer
        let mut pre_outputs: Vec<f32> = Vec::with_capacity(output_size);
        for i in 0..output_size {
            pre_outputs.push(self.output[i].compute(&hidden));
        }

        // Apply activation function on output layer
        return match self.activation {
            Activation::Softmax => {
                let mut exps = Vec::with_capacity(output_size);
                for i in 0..pre_outputs.len() {
                    exps.push(pre_outputs[i].exp());
                }
                let sum: f32 = exps.iter().sum();
                let mut max = 0.0;
                let mut max_index: u8 = 0;
                let mut outputs = Vec::with_capacity(output_size);
                for i in 0..pre_outputs.len() {
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