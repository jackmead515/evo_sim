use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Point(pub f32, pub f32);

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Block(pub Point, pub Point, pub Point, pub Point);

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Bounds {
    pub blocks: Vec<Block>,
    pub width: usize,
    pub height: usize,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Traits {
    pub restitution: f32,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Activation {
    Sigmoid,
    Softmax
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Neuron {
    pub bias: f32,
    pub weights: Vec<f32>,
    pub activation: Activation,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Brain {
    pub hidden: Vec<Neuron>,
    pub output: Vec<Neuron>,
    pub activation: Activation,
    pub output_buffer: Vec<f32>,
    pub hidden_buffer: Vec<f32>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Creature {
    pub creature_id: usize,
    pub brain: Brain,
    pub traits: Traits,
    pub bounds: Bounds,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CreatureState {
    pub creature_id: usize,
    pub bounds: Bounds
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Wall {
    pub bounds: Bounds
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Step {
    pub step_id: usize,
    pub states: HashMap<usize, CreatureState>,
    pub dynamic_walls: Vec<Wall>
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Cycle {
    pub cycle_id: usize,
    pub creatures: HashMap<usize, Creature>,
    pub walls: Vec<Wall>,
    pub steps: Vec<Step>
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Constants {
    pub max_cycles: usize,
    pub max_steps: usize,
    pub creature_amount: usize,
    pub brain_size: usize,
    pub input_size: usize,
    pub output_size: usize,
    pub block_amount: usize,
    pub block_size: f32
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Simulation {
    pub simulation_id: usize,
    pub constants: Constants,
    pub cycles: Vec<Cycle>
}

