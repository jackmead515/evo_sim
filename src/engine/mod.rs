
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

pub fn perform_cycle(simulation: &Simulation, cycle: &mut Cycle) {

  let mut range = rand::thread_rng();
  let mut rigid_body_set = RigidBodySet::new();
  let mut collider_set = ColliderSet::new();

  for (body, collider) in create::world_colliders(&simulation.constants) {
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
          for (creature_id, mut creature) in cycle.creatures.iter_mut() {
            let (mut body, mut collider) = create::dynamic_body(simulation.constants.block_size, &creature.bounds);
        
            body.user_data = *creature_id as u128;
            collider.user_data = *creature_id as u128;
            collider.set_restitution(creature.traits.restitution);
            collider.set_friction(creature.traits.friction);
            creature.traits.mass = body.mass();

            let translation = (range.gen_range(50.0, 550.0), range.gen_range(50.0, 550.0));

            body.set_translation(vector![translation.0, translation.1], true);
        
            let body_handle = rigid_body_set.insert(body);
            collider_set.insert_with_parent(collider, body_handle, &mut rigid_body_set);

            step.states.insert(*creature_id, CreatureState {
              creature_id: *creature_id,
              translation: Point { x: translation.0, y: translation.1 },
              stamina: creature.traits.stamina,
              rotation: 0.0,
              decision: 0,
            });
          }
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

        for (_body_handle, body) in rigid_body_set.iter_mut() {
          let creature_id = body.user_data as u32;
          let translation = body.translation();
          let rotation = body.rotation().angle();

          let creature = cycle.creatures.get_mut(&creature_id).unwrap();

          let creature_state = step.states.get_mut(&creature_id).unwrap();
          creature_state.translation = Point { x: translation.x, y: translation.y };
          creature_state.rotation = rotation;

          let (_outputs, decision) = creature.brain.compute(&vec![
            creature_state.translation.x,
            creature_state.translation.y,
            creature_state.rotation,
            creature_state.stamina,
            0.0
          ]);

          let net_speed = creature.traits.get_net_speed();
          let stamina_factor = creature.traits.get_stamina_factor();

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

          if creature_id == 5 {
            print!("{:?}", creature_state);
            println!("\t{}, {}", net_speed, stamina_factor);
          }
        }

        info!("computed simulation step {} in {} ms", step.step_id, now.elapsed().as_millis());

        cycle.steps.push(step);
      },
      None => {
        break;
      }
    }
    
  }
}

pub fn set_parameters() {

}

pub fn get_cycle() {

}