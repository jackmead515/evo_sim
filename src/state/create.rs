use std::f32;
use std::time::{Instant};

use rand::Rng;
use rapier2d::prelude::*;
use rapier2d::math::Point;
use log::{info};

use crate::state::models::*;
use crate::state::bounds;

// pub fn verts_to_hull(verts: Vec<Vec<f32>>) -> Hull {
//     let mut verticies = Vec::new();

//     for v in verts {
//         verticies.push(Vertex { points: vec![v[0] as f64, v[1] as f64] });
//     }

//     return Hull {
//         verticies: verticies
//     }
// }

// pub fn convex_hull(point_amount: usize, size: f32, uniform: f32) -> Vec<Vertex> {
//     let mut rando = rand::thread_rng();
//     let two_pi = 2.0 * f32::consts::PI;
//     let step_size = two_pi / point_amount as f32;

//     let mut angle = 0.0;
//     let mut points: Vec<Vertex> = Vec::new();

//     while angle < two_pi {
//         let y = (angle.sin() * size + rando.gen_range(-uniform, uniform)) as f64;
//         let x = (angle.cos() * size + rando.gen_range(-uniform, uniform)) as f64;
//         points.push(Vertex { points: vec![x, y] });
//         angle += step_size;
//     }

//     return points;
// }

pub fn creature() -> (Creature, RigidBody, Collider) {
    let now = Instant::now();
    let id = 1;
    let block_amount = 10;
    let block_size = 2.5;

    
    let bounds = bounds::random_bounds(block_size, block_amount);
    
    let (mut body, mut collider) = dynamic_body(block_size, &bounds);

    body.user_data = id as u128;
    collider.user_data = id as u128;

    let creature = Creature {
        creature_id: id,
        bounds: bounds
    };

    info!("generated creature: {} ms.", now.elapsed().as_millis());

    return (creature, body, collider);
}

pub fn dynamic_body(block_size: f32, bounds: &Bounds) -> (RigidBody, Collider) {

    let mut verts: Vec<Vec<f32>> = Vec::new();
    for block in &bounds.blocks {
        verts.extend(block.to_verts());
    }

    let path: Vec<Point<Real>> = verts.iter()
        .map(|vert| Point::new(vert[0] as f32, vert[1] as f32))
        .collect();

    // let indices = (0..path.len() - 1)
    //     .map(|i| [i as u32, i as u32 + 1])
    //     .collect::<Vec<_>>();
  
    let rigid_body = RigidBodyBuilder::new_dynamic()
        .ccd_enabled(true)
        .build();
  
    //let mut force: Vec<Point<Real>> = Vec::new();
    //force.push(Point::new(0.0, 0.0));
    //force.push(Point::new(1.0, 1.0));
    // rigid_body.apply_force_at_point(
    //   vector![0.0, 100.0],
    //   Point::new(0.5, 0.5),
    //   true
    // );

    let half = block_size / 2.0;

    let mut shapes = Vec::new();
    for vert in verts.iter().step_by(4) {
        shapes.push((
            Isometry::translation(vert[0], vert[1]),
            SharedShape::cuboid(0.1, 0.1)
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