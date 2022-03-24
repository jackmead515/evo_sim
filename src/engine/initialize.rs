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

use crate::state::models::{Cycle, Step, CreatureState, Point};
use crate::state::simulation::{Constants, Simulation};
use crate::state::GeneExpression;
use crate::engine;

fn generate_x_coordinates(total_coordinates: usize) -> Vec<u32> {
  let mut lmax = 0;
  let mut cmax = 0;
  let mut coords = Vec::with_capacity(total_coordinates);

  for i in 0..total_coordinates {
    if i == 0 {
      coords.push(0);
      coords.push(1);
      lmax = 2;
    } else if cmax < lmax {
      coords.push(cmax);
      cmax += 1;
    } else if cmax == lmax {
      for _ in 0..cmax {
        coords.push(cmax);
      }
      lmax = cmax + 1;
      cmax = 0;
    }
  }

  return coords;
}

fn generate_y_coordinates(total_coordinates: usize) -> Vec<u32> {
  let mut lmax = 0;
  let mut cmax = 0;
  let mut coords = Vec::with_capacity(total_coordinates);

  for i in 0..total_coordinates {
    if i == 0 {
      coords.push(0);
      coords.push(0);
      coords.push(1);
      coords.push(1);
      lmax = 2;
    } else if cmax < lmax {
      coords.push(cmax);
      cmax += 1;
    } else if cmax == lmax {
      for _ in 0..cmax+1 {
        coords.push(cmax);
      }
      lmax = cmax + 1;
      cmax = 0;
    }
  }

  return coords;
}

fn generate_placement_coordinates(total_coordinates: usize) -> Vec<Point> {
  let xs = generate_x_coordinates(total_coordinates);
  let ys = generate_y_coordinates(total_coordinates);

  let mut coords = Vec::with_capacity(total_coordinates);

  for i in 0..total_coordinates {
    coords.push(Point { x: xs[i] as f32, y: ys[i] as f32 });
  }

  return coords;
}

/// Create the creates randomly in the world
pub fn initialize_creatures(
    constants: &Constants,
    cycle: &mut Cycle,
    step: &mut Step,
    rigid_body_set: &mut RigidBodySet,
    collider_set: &mut ColliderSet,
    range: &mut rand::prelude::ThreadRng,
) {
  let coordinates = generate_placement_coordinates(cycle.creatures.len());
  let mut max_creature_width = 0.0;
  let mut max_creature_height = 0.0;

  for (creature_id, mut creature) in cycle.creatures.iter_mut() {
    let (mut body, mut collider) = engine::create::dynamic_body(constants.block_size, &creature.bounds);

    if *creature_id == 0 {
      println!("{:?}", creature.bounds);
    }

    let total_creature_width = creature.bounds.width * constants.block_size;
    let total_creature_height = creature.bounds.height * constants.block_size;
    if total_creature_width > max_creature_width {
      max_creature_width = total_creature_width;
    }
    if total_creature_height > max_creature_height {
      max_creature_height = total_creature_height;
    }

    // set the basic collider and body attributes
    body.user_data = *creature_id as u128;
    collider.user_data = *creature_id as u128;
    collider.set_restitution(creature.traits.restitution);
    collider.set_friction(creature.traits.friction);

    // set the block amount of the creature so we can get the net mass property
    creature.traits.block_amount = creature.bounds.blocks.len() as u32;

    let gene_codes = creature.gene_codes(constants);

    // set the creature traits based off of it's genes
    creature.traits.color = creature.rgba_codes(&gene_codes);
    creature.traits.gene_codes = creature.ascii_codes(&gene_codes);

    let mass_props = collider.mass_properties();
    body.set_mass_properties(
      MassProperties::new(
        mass_props.local_com,
        creature.traits.get_net_mass(),
        0.0
      ),
      true
    );

    // insert the physics body into the set
    let body_handle = rigid_body_set.insert(body);
    collider_set.insert_with_parent(collider, body_handle, rigid_body_set);
    
    // insert the creature state into the state map
    let coordinate = coordinates.get(*creature_id as usize).unwrap();
    step.states.insert(*creature_id, CreatureState {
      creature_id: *creature_id,
      translation: Point { x: coordinate.x, y: coordinate.y },
      stamina: creature.traits.stamina,
      rotation: 0.0,
      decision: 0,
    });
  }

  // as extra padding between creatures
  max_creature_width += constants.block_size;
  max_creature_height += constants.block_size;

  let total_placement_width = max_creature_width * cycle.creatures.len() as f32;
  let total_placement_height = max_creature_height * cycle.creatures.len() as f32;
  let position_x = (constants.world_width as f32 - total_placement_width) / 2.0;
  let position_y = (constants.world_height as f32 - total_placement_height) / 2.0;

  println!("tpw{}, tph{}", total_placement_width, total_placement_height);
  println!("px{}, py{}", position_x, position_y);

  for (_body_handle, body) in rigid_body_set.iter_mut() {
    let creature_id = body.user_data as u32;

    // use if let Some statement incase the creature_id does
    // not exist. I.E: some boundary or something.
    if let Some(creature) = cycle.creatures.get_mut(&creature_id) {
      let creature_state = step.states.get_mut(&creature_id).unwrap();
      let coordinate = coordinates.get(creature_id as usize).unwrap();

      let x = (coordinate.x * max_creature_width) + position_x;
      let y = (coordinate.y * max_creature_height) + position_y;

      println!("cx{}, cy{}, x{}, y{}", coordinate.x, coordinate.y, x, y);

      creature_state.translation = Point { x: x, y: y };
      body.set_translation(vector![x, y], true);
    }
  }
}