extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;
extern crate rand;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL, GlyphCache, TextureSettings};
use piston::event_loop::{EventSettings, EventLoop, Events};
use piston::input::{Button, MouseButton, RenderArgs, MouseCursorEvent, PressEvent, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;
use rand::Rng;

use rapier2d::prelude::*;
use rapier2d::math::Point;

pub mod creature;
pub mod game;
pub mod genes;
pub mod interface;
pub mod draw;
pub mod brain;
pub mod io;
pub mod physics;
pub mod constants;

use crate::game::Game;
use crate::creature::Creature;

fn main() {
    let opengl = OpenGL::V3_2;

    let window_width = constants::get_window_width();
    let window_height = constants::get_window_height();

    let mut window: Window = WindowSettings::new("evo_sim", [
        window_width as u32,
        window_height as u32
    ])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();
    
    let font = "/System/Library/Fonts/NewYork.ttf";
    let mut glyphs = GlyphCache::new(font, (), TextureSettings::new()).unwrap();
    let mut game = Game::new(opengl);
    let mut rando = rand::thread_rng();

    let mut rigid_body_set = RigidBodySet::new();
    let mut collider_set = ColliderSet::new();

    let mut rigid_body = RigidBodyBuilder::new_dynamic()
        .translation(vector![0.0, 0.0])
        .build();

    let mut collider = ColliderBuilder::triangle(
        Point::new(0.0, 0.0),
        Point::new(0.0, 1.0),
        Point::new(1.0, 1.0),
    )
    .restitution(0.7)
    .friction(0.1)
    .build();

    let body_handle = rigid_body_set.insert(rigid_body);

    collider_set.insert_with_parent(collider, body_handle, &mut rigid_body_set);

    let gravity = vector![0.0, 0.0];
    let integration_parameters = IntegrationParameters::default();
    let mut physics_pipeline = PhysicsPipeline::new();
    let mut island_manager = IslandManager::new();
    let mut broad_phase = BroadPhase::new();
    let mut narrow_phase = NarrowPhase::new();
    let mut joint_set = JointSet::new();
    let mut ccd_solver = CCDSolver::new();
    let physics_hooks = ();
    let event_handler = ();

    for _ in 0..200 {
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
    
        let ball_body = &rigid_body_set[body_handle];
        println!(
          "Ball altitude: {}",
          ball_body.translation().y
        );
      }

    // for id in 0..200 {
    //     let mut creature = Creature::new(
    //         id,
    //         constants::BRAIN_SIZE,
    //         constants::INPUTS_SIZE,
    //         constants::OUTPUTS_SIZE,
    //     );
    //     let rand_x: isize = rando.gen_range(50isize, window_width - 50isize);
    //     let rand_y: isize = rando.gen_range(50isize, window_height - 50isize);
    //     creature.set_center(rand_x, rand_y);
    //     game.world.add_creature(creature);
    // }

    // let mut events = Events::new(EventSettings::new());
    // events.set_max_fps(30);
    // events.set_ups(30);

    // while let Some(e) = events.next(&mut window) {
    //     if let Some(args) = e.mouse_cursor_args() {
    //         game.mouse_pos = [args[0] as isize, args[1] as isize];
    //         game.mouse_input();
    //     }
    //     if let Some(Button::Mouse(MouseButton::Left)) = e.press_args() {
    //         game.mouse_press();
    //     }
    //     if let Some(args) = e.render_args() {
    //         game.frames += 1;
    //         game.render(&args);
    //     }
    //     if let Some(args) = e.update_args() {
    //         game.updates += 1;
    //         game.elapsed += args.dt;
    //         if game.elapsed > 1.0 {
    //             let fps = (game.frames as f64) / game.elapsed;
    //             println!("frames: {} | updates: {} | fps: {}", game.frames, game.updates, fps);
    //             game.frames = 0;
    //             game.updates = 0;
    //             game.elapsed = 0.0;
    //         }
    //         game.update(&args);
            
    //     }
    // }
}
