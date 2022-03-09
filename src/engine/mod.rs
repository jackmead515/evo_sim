
use std::fs::File;
use std::io::prelude::*;
use std::time::{Instant};
use std::collections::HashMap;
use std::thread;
use std::thread::{JoinHandle};
use std::sync::{Arc, Mutex};
use std::sync::mpsc::channel;

use rand::Rng;
use rand;
use rapier2d::prelude::*;
use log::{info};

use crate::state::models::{Cycle, CreatureState, Point};
use crate::state::simulation::Simulation;

pub mod create;
pub mod cycle;
pub mod render;

pub fn set_parameters() {

}

pub fn get_cycle() {

}