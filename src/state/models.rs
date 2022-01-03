use std::collections::HashMap;

pub struct Point {
    pub x: f32,
    pub y: f32
}

pub struct Block(pub Point, pub Point, pub Point, pub Point);

pub struct Bounds {
    pub blocks: Vec<Block>,
    pub width: usize,
    pub height: usize,
}

pub struct Traits {
    pub creature_id: usize,
    pub color: [usize; 4],
}

pub enum Activation {
    Sigmoid,
    Softmax
}

pub struct Neuron {
    pub bias: f32,
    pub weights: Vec<f32>,
    pub activation: Activation,
}

pub struct Brain {
    pub creature_id: usize,
    pub hidden: Vec<Neuron>,
    pub output: Vec<Neuron>,
    pub activation: Activation,
    pub output_buffer: Vec<f32>,
    pub hidden_buffer: Vec<f32>,
}

pub struct Creature {
    pub creature_id: usize,
    pub bounds: Bounds
}

pub struct Wall {
    pub bounds: Bounds
}

pub struct Step {
    pub step_id: usize,
    pub creatures: Vec<Creature>,
    pub dynamic_walls: Vec<Wall>
}

pub struct Cycle {
    pub cycle_id: usize,
    pub brain_map: HashMap<usize, Brain>,
    pub trait_map: HashMap<usize, Traits>,
    pub walls: Vec<Wall>,
    pub steps: Vec<Step>
}

pub struct Constants {
    pub creature_amount: usize,
    pub brain_size: usize,
    pub input_size: usize,
    pub output_size: usize,
    pub block_amount: usize,
    pub block_size: f32
}

pub struct Simulation {
    pub simulation_id: usize,
    pub constants: Constants,
    pub cycles: Vec<Cycle>
}