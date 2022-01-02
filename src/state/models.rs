use std::collections::HashMap;

pub struct Point {
    pub x: f32,
    pub y: f32
}

pub struct Block(pub Point, pub Point, pub Point, pub Point);

pub struct Bounds {
    pub blocks: Vec<Block>
}

pub struct Color {
    pub rgba: [usize; 4]
}

pub struct Traits {
    pub creature_id: usize,
    pub color: Color,
}

pub struct Brain {
    pub creature_id: usize,
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


pub struct Simulation {
    pub simulation_id: usize,
    pub cycle: Vec<Cycle>
}