
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;

use crate::state::models::{Cycle, CreatureState, Point};
use crate::state::simulation::Simulation;
use crate::engine;

pub fn run(simulation: &Simulation, cycle: &Cycle) {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Create an Glutin window.
    let mut window: Window = WindowSettings::new(
        "Simulation", [simulation.constants.world_width, simulation.constants.world_height]
    )
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();
    
    let mut gl = GlGraphics::new(opengl);

    const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
    const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];

    let mut step_index = 0;

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        use graphics::*;

        if let Some(args) = e.render_args() {
            let step = cycle.steps.get(step_index).unwrap();

            for (creature_id, creature) in cycle.creatures.iter() {
                let state = step.states.get(creature_id).unwrap();

                let tx = state.translation.x;
                let ty = state.translation.y;
                let rc = state.rotation.cos();
                let rs = state.rotation.sin();

                for block in creature.bounds.blocks.iter() {
                    let x = block.position.x * block.size;
                    let y = block.position.y * block.size;

                    //let x1 = (x*rc - y*rs) + tx;
                    //let y1 = (x*rs + y*rs) + ty;

                    let square = rectangle::square(0.0, 0.0, block.size as f64);

                    gl.draw(args.viewport(), |c, gl| {
                        // Clear the screen.
                        clear(BLACK, gl);
            
                        let transform = c
                            .transform
                            .trans(x as f64, y as f64)
                            .rot_rad(state.rotation as f64);
            
                        // Draw a box rotating around the middle of the screen.
                        rectangle(GREEN, square, transform, gl);
                    });
                }
            }

            step_index += 1;
            if step_index >= cycle.steps.len() {
                break;
            }
        }
    }

}
