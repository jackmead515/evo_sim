
use std::fs::File;
use std::io::prelude::*;
use std::time::{Instant};

use rapier2d::prelude::*;
use log::{info};

use crate::state;

pub static WORLD_WIDTH: f32 = 800.0;
pub static WORLD_HEIGHT: f32 = 640.0;

pub fn get_world_colliders() -> Vec<(RigidBody, Collider)> {

  let thickness = 10.0;

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

  // let ground = ColliderBuilder::cuboid(WORLD_WIDTH, thickness)
  //   .translation(vector![0.0, WORLD_HEIGHT + thickness])
  //   .build();

  // let ceiling = ColliderBuilder::cuboid(WORLD_WIDTH, thickness)
  //   .translation(vector![0.0, -thickness])
  //   .build();

  // let left_wall = ColliderBuilder::cuboid(thickness, WORLD_HEIGHT)
  //   .translation(vector![-thickness, 0.0])
  //   .build();
  
  // let right_wall = ColliderBuilder::cuboid(thickness, WORLD_HEIGHT)
  //   .translation(vector![WORLD_WIDTH + thickness, 0.0])
  //   .build();

  // return vec![
  //   (RigidBodyBuilder::new_static().build(), ground),
  //   (RigidBodyBuilder::new_static().build(), ceiling),
  //   (RigidBodyBuilder::new_static().build(), left_wall),
  //   (RigidBodyBuilder::new_static().build(), right_wall)
  // ]
}

pub fn perform_cycle() {
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

  let mut rigid_body_set = RigidBodySet::new();
  let mut collider_set = ColliderSet::new();

  let world_colliders = get_world_colliders();
  for (body, collider) in world_colliders {
    let body_handle = rigid_body_set.insert(body);
    collider_set.insert_with_parent(collider, body_handle, &mut rigid_body_set);
  }

  let (creature, mut body, mut collider) = state::create::creature();

  body.set_body_type(RigidBodyType::Dynamic);
  body.set_translation(vector![WORLD_WIDTH / 2.0, WORLD_HEIGHT / 2.0], true);
  body.set_linvel(vector![50.0, -50.0], true);

  let body_handle = rigid_body_set.insert(body);
  collider_set.insert_with_parent(collider, body_handle, &mut rigid_body_set);

  let gravity = vector![0.0, 9.81];
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

  let mut file = File::create("./cycle.txt").unwrap();

  info!("Starting physics pipeline");

  for index in 0..1000 {
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

    let body = rigid_body_set.get_mut(body_handle).unwrap();
    //let collider_handle = body.colliders()[0];
    //let collider = collider_set.get_mut(collider_handle).unwrap();

    if index <= 1 {
      body.apply_force_at_point(
        vector![0.0, 5.0],
        Point::new(2.0, 2.0),
        true
      );
    } 

    let translation = body.translation();
    let rotation = body.rotation().angle();
    //let rotation = collider.rotation().angle();

    let mut translated: Vec<Vec<f32>> = Vec::new();

    for block in &creature.bounds.blocks {
      for vert in block.to_verts() {
        let vec = vector![vert[0], vert[1]];
        //let iso = Isometry2::new(vec, rotation as f64);
        //let x = iso.translation.x + translation.x as f64;
        //let y = iso.translation.y + translation.y as f64;
        let sin = rotation.sin();
        let cos = rotation.cos();
        let x = (vec[0]*cos - vec[1]*sin) + translation.x;
        let y = (vec[0]*sin + vec[1]*cos) + translation.y;
        translated.push(vec![x, y]);
      }
    }
  
    let body = format!("{:?}\n", translated);
    file.write_all(body.as_bytes()).unwrap();
  }
}

pub fn set_parameters() {

}

pub fn get_cycle() {

}