use std::f32;
use std::time::{Instant};

use rand::Rng;
use rapier2d::prelude::*;
use rapier2d::math::Point;
use log::{info};

use crate::state::models::*;
use crate::state::simulation::Constants;

pub fn world_colliders(constants: &Constants) -> Vec<(RigidBody, Collider)> {
  let path = vec![
    Point::new(0.0, 0.0),
    Point::new(0.0, constants.world_height as f32),
    Point::new(constants.world_width as f32, constants.world_height as f32),
    Point::new(constants.world_width as f32, 0.0),
    Point::new(0.0, 0.0)
  ];

  let indices = (0..path.len() - 1)
        .map(|i| [i as u32, i as u32 + 1])
        .collect::<Vec<_>>();

  let collider = ColliderBuilder::polyline(path, Option::Some(indices))
    .build();

  return vec![
    (RigidBodyBuilder::new_static().build(), collider)
  ];
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
        Isometry::new(vector![
          block.position.x*block_size + half_size,
          block.position.y*block_size + half_size
        ], 0.0),
        SharedShape::cuboid(half_size, half_size)
      ));
    }

    let mut collider = ColliderBuilder::compound(shapes).density(0.5).build();

    // let collider = ColliderBuilder::convex_hull(&path)
    //     .unwrap()
    //     .build();
  
    // let collider = ColliderBuilder::convex_decomposition(&path, &indices)
    //     .build();
  
    return (rigid_body, collider);
}