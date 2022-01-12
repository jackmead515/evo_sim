
use std::fs::File;
use std::io::prelude::*;
use std::time::{Instant};
use std::collections::HashMap;

use rand::Rng;
use rand;
use rapier2d::prelude::*;
use log::{info};

use crate::state::models::{Simulation, Cycle, Step, CreatureState};

pub mod create;

pub fn perform_cycle(simulation: &Simulation, cycle: &mut Cycle) {
  // let max_steps = 1000;

  // for step in 0..max_steps {
  //   for creature in creatures {
  //     let decisions = creature.decide();
  //     creature.update_physics(decisions);
  //   }

  //   physics.step();
  //   creature.update_state();
  //   save_cycle_step();
  //   evolve();
  // }

  // let world_width = 1000.0;
  // let world_height = 1000.0;

  let mut range = rand::thread_rng();

  let mut rigid_body_set = RigidBodySet::new();
  let mut collider_set = ColliderSet::new();

  for (body, collider) in create::world_colliders() {
    let body_handle = rigid_body_set.insert(body);
    collider_set.insert_with_parent(collider, body_handle, &mut rigid_body_set);
  }

  // let (mut step, bodies) = simulation.next_step();

  // let mut i = 0;
  // for (mut body, collider) in bodies {
  //   i += 1;
  //   body.set_translation(vector![(i as f32 * 50.0), WORLD_HEIGHT / 2.0], true);
  //   body.set_linvel(vector![range.gen_range(-50.0, 50.0), range.gen_range(-50.0, 50.0)], true);
  //   let body_handle = rigid_body_set.insert(body);
  //   collider_set.insert_with_parent(collider, body_handle, &mut rigid_body_set);
  // }
  
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

  //let mut file = File::create("./cycle.txt").unwrap();

  info!("Starting physics pipeline");

  let mut steps: Vec<Step> = Vec::new();

  loop {
    let now = Instant::now();
    match cycle.next_step(&simulation.constants) {
      Some(mut step) => {

        if step.step_id == 0 {
          for (creature_id, creature) in cycle.creatures.iter() {
            let (mut body, mut collider) = create::dynamic_body(simulation.constants.block_size, &creature.bounds);
        
            body.user_data = *creature_id as u128;
            collider.user_data = *creature_id as u128;
            collider.set_restitution(creature.traits.restitution);

            let translation = (range.gen_range(50.0, 550.0), range.gen_range(50.0, 550.0));

            body.set_translation(vector![translation.0, translation.1], true);
        
            let body_handle = rigid_body_set.insert(body);
            collider_set.insert_with_parent(collider, body_handle, &mut rigid_body_set);

            step.states.insert(*creature_id, CreatureState {
              creature_id: *creature_id,
              bounds: creature.bounds.translate(translation.0, translation.1, 0.0),
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
          creature_state.bounds = creature.bounds.translate(translation.x, translation.y, rotation);;

          let (_outputs, decision) = creature.brain.compute(&vec![0.1, 0.2, 0.3, 0.4, 0.5]);

          if decision == 0 {
            body.apply_force(vector![0.0, -1.0], true);
          } else if decision == 1 {
            body.apply_force(vector![0.0, 1.0], true);
          } else if decision == 2 {
            body.apply_force(vector![-1.0, 0.0], true);
          } else if decision == 3 {
            body.apply_force(vector![1.0, 0.0], true);
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