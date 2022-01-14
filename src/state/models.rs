//*
///simulations
///cycles
///step
///creatures
///states
///brains
///neurons
///traits

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Point {
    #[prost(float, required, tag="1")]
    pub x: f32,
    #[prost(float, required, tag="2")]
    pub y: f32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Block {
    #[prost(message, required, tag="1")]
    pub position: Point,
    #[prost(float, required, tag="2")]
    pub width: f32,
    #[prost(float, required, tag="3")]
    pub height: f32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Bounds {
    #[prost(message, repeated, tag="1")]
    pub blocks: ::prost::alloc::vec::Vec<Block>,
    #[prost(uint32, required, tag="2")]
    pub width: u32,
    #[prost(uint32, required, tag="3")]
    pub height: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Traits {
    #[prost(float, required, tag="1")]
    pub restitution: f32,
    #[prost(float, required, tag="2")]
    pub friction: f32,
    #[prost(float, required, tag="3")]
    pub stamina: f32,
    #[prost(float, required, tag="4")]
    pub mass: f32,
    #[prost(float, required, tag="5")]
    pub strength: f32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Neuron {
    #[prost(enumeration="Activation", required, tag="1")]
    pub activation: i32,
    #[prost(float, required, tag="2")]
    pub bias: f32,
    #[prost(float, repeated, packed="false", tag="3")]
    pub weights: ::prost::alloc::vec::Vec<f32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Brain {
    #[prost(enumeration="Activation", required, tag="1")]
    pub activation: i32,
    #[prost(message, repeated, tag="2")]
    pub hidden: ::prost::alloc::vec::Vec<Neuron>,
    #[prost(message, repeated, tag="3")]
    pub output: ::prost::alloc::vec::Vec<Neuron>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Creature {
    #[prost(uint32, required, tag="1")]
    pub creature_id: u32,
    #[prost(message, required, tag="2")]
    pub brain: Brain,
    #[prost(message, required, tag="3")]
    pub traits: Traits,
    #[prost(message, required, tag="4")]
    pub bounds: Bounds,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreatureState {
    #[prost(uint32, required, tag="1")]
    pub creature_id: u32,
    #[prost(message, required, tag="2")]
    pub translation: Point,
    #[prost(float, required, tag="3")]
    pub rotation: f32,
    #[prost(float, required, tag="4")]
    pub stamina: f32,
    #[prost(uint32, required, tag="5")]
    pub decision: u32
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Wall {
    #[prost(message, required, tag="1")]
    pub bounds: Bounds,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Step {
    #[prost(uint32, required, tag="1")]
    pub step_id: u32,
    #[prost(map="uint32, message", tag="2")]
    pub states: ::std::collections::HashMap<u32, CreatureState>,
    #[prost(message, repeated, tag="3")]
    pub dynamic_walls: ::prost::alloc::vec::Vec<Wall>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Cycle {
    #[prost(uint32, required, tag="1")]
    pub cycle_id: u32,
    #[prost(map="uint32, message", tag="2")]
    pub creatures: ::std::collections::HashMap<u32, Creature>,
    #[prost(message, repeated, tag="3")]
    pub walls: ::prost::alloc::vec::Vec<Wall>,
    #[prost(message, repeated, tag="4")]
    pub steps: ::prost::alloc::vec::Vec<Step>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Activation {
    Sigmoid = 1,
    Softmax = 2,
}
