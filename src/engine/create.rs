use std::f32;
use std::time::{Instant};

use rand::Rng;
use rapier2d::prelude::*;
use rapier2d::math::Point;
use log::{info};

use crate::state::models::*;

pub static WORLD_WIDTH: f32 = 600.0;
pub static WORLD_HEIGHT: f32 = 600.0;

pub fn world_colliders() -> Vec<(RigidBody, Collider)> {
  let path = vec![
    Point::new(0.0, 0.0),
    Point::new(0.0, WORLD_HEIGHT),
    Point::new(WORLD_WIDTH, WORLD_HEIGHT),
    Point::new(WORLD_WIDTH, 0.0),
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
            Isometry::new(vector![block.p1.x, block.p1.y], 0.0),
            SharedShape::cuboid(half_size, half_size)
        ));
    }

    let mut collider = ColliderBuilder::compound(shapes).build();

    // let collider = ColliderBuilder::convex_hull(&path)
    //     .unwrap()
    //     .build();
  
    // let collider = ColliderBuilder::convex_decomposition(&path, &indices)
    //     .build();
  
    return (rigid_body, collider);
}