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
use log::{info, debug};

use rapier2d::dynamics::RigidBodySet;
use rapier2d::geometry::ColliderSet;

use crate::state::models::{Cycle, Step, Point};
use crate::state::simulation::{Constants, Simulation};
use crate::engine;

pub fn run(simulation: &Simulation, cycle: &mut Cycle) {

    let mut range = rand::thread_rng();
    let mut rigid_body_set = RigidBodySet::new();
    let mut collider_set = ColliderSet::new();
  
    for (mut body, collider) in engine::create::world_colliders(&simulation.constants) {
      // TODO: need to set arbitarly high value. The creatures user_data
      // is allocated from 0 starting counting up. The user_data defaults to 0!
      body.user_data = range.gen_range(10000, 20000) as u128;
      let body_handle = rigid_body_set.insert(body);
      collider_set.insert_with_parent(collider, body_handle, &mut rigid_body_set);
    }
    
    let gravity = vector![0.0, 0.0];
    let mut integration_parameters = IntegrationParameters::default();
    integration_parameters.dt = 1.0 / 60.0;
    integration_parameters.max_ccd_substeps = 1;
  
    let mut physics_pipeline = PhysicsPipeline::new();
    let mut island_manager = IslandManager::new();
    let mut broad_phase = BroadPhase::new();
    let mut narrow_phase = NarrowPhase::new();
    let mut joint_set = JointSet::new();
    let mut ccd_solver = CCDSolver::new();
    let physics_hooks = ();
    let event_handler = ();
  
    info!("Starting physics pipeline");
  
    loop {
      let now = Instant::now();
      match cycle.next_step(&simulation.constants) {
        Some(mut step) => {
  
          if step.step_id == 0 {
            engine::initialize::initialize_creatures(
              &simulation.constants,
              cycle,
              &mut step,
              &mut rigid_body_set,
              &mut collider_set,
              &mut range
            );
          }
  
          physics_pipeline.step(
            &gravity,
            &integration_parameters,
            &mut island_manager,
            &mut broad_phase,
            &mut narrow_phase,
            &mut rigid_body_set,
            &mut collider_set,
            &mut joint_set,
            &mut ccd_solver,
            &physics_hooks,
            &event_handler,
          );
  
          update_creatures(
            &simulation.constants,
            cycle,
            &mut step,
            &mut rigid_body_set,
            &mut collider_set,
            &mut range
          );
  
          debug!("computed simulation step {} in {} ms", step.step_id, now.elapsed().as_millis());
          cycle.steps.push(step);
        },
        None => {
          break;
        }
      }
      
    }
}

pub fn update_creatures(
  constants: &Constants,
  cycle: &mut Cycle,
  step: &mut Step,
  rigid_body_set: &mut RigidBodySet,
  collider_set: &mut ColliderSet,
  range: &mut rand::prelude::ThreadRng,
) {
  for (_body_handle, body) in rigid_body_set.iter_mut() {
    let creature_id = body.user_data as u32;

    // use if let Some statement incase the creature_id does
    // not exist. I.E: some boundary or something.
    if let Some(creature) = cycle.creatures.get_mut(&creature_id) {

      // update the translation and rotation of the state
      let creature_state = step.states.get_mut(&creature_id).unwrap();
      let translation = body.translation();
      let rotation = body.rotation().angle();

      creature_state.translation = Point { x: translation.x, y: translation.y };
      creature_state.rotation = rotation;

      // gather the inputs for the creatures brain
      let mut inputs = vec![
        creature_state.translation.x,
        creature_state.translation.y,
        creature_state.rotation,
        creature_state.stamina,
        step.step_id as f32
      ];

      // compute against the inputs and get the decision
      let (outputs, decision) = creature.brain.compute(&mut inputs);
      creature_state.decision = decision as u32;
      
      let net_speed = creature.traits.get_net_speed();
      let stamina_factor = creature.traits.get_stamina_factor();

      // based on creature decision, take an action
      if decision == 0 && creature_state.stamina > stamina_factor {
        body.apply_force(vector![0.0, -net_speed], true);
        creature_state.stamina -= stamina_factor;
      } else if decision == 1 && creature_state.stamina > stamina_factor {
        body.apply_force(vector![0.0, net_speed], true);
        creature_state.stamina -= stamina_factor;
      } else if decision == 2 && creature_state.stamina > stamina_factor {
        body.apply_force(vector![-net_speed, 0.0], true);
        creature_state.stamina -= stamina_factor;
      } else if decision == 3 && creature_state.stamina > stamina_factor {
        body.apply_force(vector![net_speed, 0.0], true);
        creature_state.stamina -= stamina_factor;
      }

      if creature_id == 0 {
        println!("{:?}", creature_state);
        // info!("inputs: {:?}, outputs: {:?}, decision: {}", inputs, outputs, decision);
        // println!("\tspeed {}, sfactor {}", net_speed, stamina_factor);
      }
    }
  }
}