#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Point {
    #[prost(float, required, tag="1")]
    pub x: f32,
    #[prost(float, required, tag="2")]
    pub y: f32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Dimension {
    #[prost(float, required, tag="1")]
    pub width: f32,
    #[prost(float, required, tag="2")]
    pub height: f32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Translation {
    #[prost(message, required, tag="1")]
    pub translation: Point,
    #[prost(float, required, tag="2")]
    pub rotation: f32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Block {
    #[prost(message, required, tag="1")]
    pub position: Point,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Bounds {
    #[prost(message, repeated, tag="1")]
    pub blocks: ::prost::alloc::vec::Vec<Point>,
    #[prost(message, required, tag="2")]
    pub dimensions: Dimension,
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
    pub block_mass: f32,
    #[prost(uint32, required, tag="5")]
    pub block_amount: u32,
    #[prost(float, required, tag="6")]
    pub strength: f32,
    #[prost(float, repeated, packed="false", tag="7")]
    pub color: ::prost::alloc::vec::Vec<f32>,
    #[prost(string, repeated, tag="8")]
    pub gene_codes: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(float, required, tag="9")]
    pub block_size: f32,
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
    pub translation: Translation,
    #[prost(float, required, tag="3")]
    pub stamina: f32,
    #[prost(uint32, required, tag="4")]
    pub decision: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Boundary {
    #[prost(message, required, tag="1")]
    pub position: Point,
    #[prost(message, required, tag="2")]
    pub dimensions: Dimension,
    #[prost(message, required, tag="3")]
    pub translation: Translation,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Step {
    #[prost(uint32, required, tag="1")]
    pub step_id: u32,
    #[prost(map="uint32, message", tag="2")]
    pub states: ::std::collections::HashMap<u32, CreatureState>,
    #[prost(message, repeated, tag="3")]
    pub boundaries: ::prost::alloc::vec::Vec<Boundary>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Cycle {
    #[prost(uint32, required, tag="1")]
    pub cycle_id: u32,
    #[prost(map="uint32, message", tag="2")]
    pub creatures: ::std::collections::HashMap<u32, Creature>,
    #[prost(message, repeated, tag="3")]
    pub steps: ::prost::alloc::vec::Vec<Step>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Constants {
    #[prost(uint32, required, tag="1")]
    pub world_width: u32,
    #[prost(uint32, required, tag="2")]
    pub world_height: u32,
    #[prost(uint32, required, tag="3")]
    pub max_steps: u32,
    #[prost(uint32, required, tag="4")]
    pub creature_amount: u32,
    #[prost(uint32, required, tag="5")]
    pub initial_brain_size: u32,
    #[prost(uint32, required, tag="6")]
    pub max_brain_size: u32,
    #[prost(uint32, required, tag="7")]
    pub min_brain_size: u32,
    #[prost(uint32, required, tag="8")]
    pub brain_input_size: u32,
    #[prost(uint32, required, tag="9")]
    pub brain_output_size: u32,
    #[prost(uint32, required, tag="10")]
    pub initial_block_amount: u32,
    #[prost(uint32, required, tag="11")]
    pub min_block_amount: u32,
    #[prost(uint32, required, tag="12")]
    pub max_block_amount: u32,
    #[prost(float, required, tag="13")]
    pub initial_block_size: f32,
    #[prost(float, required, tag="14")]
    pub max_block_size: f32,
    #[prost(float, required, tag="15")]
    pub min_block_size: f32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Simulation {
    #[prost(uint32, required, tag="1")]
    pub simulation_id: u32,
    #[prost(message, required, tag="2")]
    pub constants: Constants,
    #[prost(uint32, repeated, packed="false", tag="3")]
    pub cycle_ids: ::prost::alloc::vec::Vec<u32>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Activation {
    Sigmoid = 1,
    Softmax = 2,
}
