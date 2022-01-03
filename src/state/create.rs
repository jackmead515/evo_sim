use std::f32;
use std::time::{Instant};

use rand::Rng;
use rapier2d::prelude::*;
use rapier2d::math::Point;
use log::{info};

use crate::state::models::*;
use crate::state::bounds;
use crate::state::brain;

pub fn simulation(id: usize) -> Simulation {
    return Simulation {
        simulation_id: id,
        cycles: Vec::new(),   
        constants: Constants {
            creature_amount: 10,
            brain_size: 50,
            input_size: 4,
            output_size: 5,
            block_amount: 10,
            block_size: 5.0
        }
    };
}

pub fn creature(id: usize, block_amount: usize, block_size: f32) -> (Creature, RigidBody, Collider) {
    let bounds = bounds::random_bounds(block_size, block_amount);
    
    let (mut body, mut collider) = dynamic_body(block_size, &bounds);

    body.user_data = id as u128;
    collider.user_data = id as u128;

    let creature = Creature {
        creature_id: id,
        bounds: bounds
    };

    return (creature, body, collider);
}

pub fn dynamic_body(block_size: f32, bounds: &Bounds) -> (RigidBody, Collider) {

    // let mut verts: Vec<Vec<f32>> = Vec::new();
    // for block in &bounds.blocks {
    //     verts.extend(block.to_verts());
    // }

    // let path: Vec<Point<Real>> = verts.iter()
    //     .map(|vert| Point::new(vert[0] as f32, vert[1] as f32))
    //     .collect();

    // let indices = (0..path.len() - 1)
    //     .map(|i| [i as u32, i as u32 + 1])
    //     .collect::<Vec<_>>();
  
    let rigid_body = RigidBodyBuilder::new_dynamic()
        .ccd_enabled(true)
        .build();

    let half_size = block_size / 2.0;

    let mut shapes = Vec::new();
    for block in bounds.blocks.iter() {
        shapes.push((
            Isometry::new(vector![block.0.x, block.0.y], 0.0),
            SharedShape::cuboid(half_size, half_size)
        ));
    }

    let mut collider = ColliderBuilder::compound(shapes).build();

    collider.set_restitution(0.3);

    // let collider = ColliderBuilder::convex_hull(&path)
    //     .unwrap()
    //     .build();
  
    // let collider = ColliderBuilder::convex_decomposition(&path, &indices)
    //     .build();
  
    return (rigid_body, collider);
}