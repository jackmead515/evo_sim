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
    output_buffer: Vec<f32>,
    hidden_buffer: Vec<f32>,
}

#[derive(Clone, Debug)]
pub struct Neuron {
    pub bias: f32,
    pub weights: Vec<f32>,
    pub activation: Activation,
}

impl Neuron {

    pub fn from_code(code: Vec<f32>) -> Self {
        let mut weights = Vec::new();
        let bias = code[0];
        let size = code[1] as usize;

        for i in 2..size {
            weights.push(code[i]);
        }

        return Neuron {
            activation: Activation::Sigmoid,
            weights: weights,
            bias: bias
        }
    }

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

    pub fn get_code(&self) -> Vec<f32> {
        let mut code: Vec<f32> = Vec::new();
        code.push(self.bias);
        code.push(self.weights.len() as f32);
        code.extend(self.weights.iter());
        return code;
    }

}

impl Brain {

    // pub fn new_from_code(code: Vec<f32>) -> Self {
    //     let brain_size = code[0] as usize;
    //     let hidden = Vec::with_capacity(brain_size as usize);

    //     for i in 1..brain_size {
    //         hidden.push(Neuron::from_code());
    //     }
    // }

    pub fn new_random(brain_size: u8, input_size: u8, output_size: u8) -> Self {
        let mut brain = Brain {
            hidden: Vec::with_capacity(brain_size as usize),
            output: Vec::with_capacity(output_size as usize),
            hidden_buffer: Vec::with_capacity(brain_size as usize),
            output_buffer: Vec::with_capacity(output_size as usize),
            activation: Activation::Softmax,
        };

        for id in 0..brain_size {
            brain.hidden.push(Neuron::random(id, input_size));
            brain.hidden_buffer.push(0.0);
        }

        for id in 0..output_size {
            brain.output.push(Neuron::random(id, brain_size));
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

    pub fn get_code(&self) -> Vec<f32> {

        let mut code: Vec<f32> = Vec::new();

        code.push(self.hidden.len() as f32);

        for neuron in self.hidden.iter() {
            code.extend(neuron.get_code().iter());
        }

        code.push(self.output.len() as f32);

        for neuron in self.output.iter() {
            code.extend(neuron.get_code().iter());
        }

        return code;
    }

}