use opengl_graphics::{GlGraphics};
use piston::input::{RenderArgs, UpdateArgs};
use opengl_graphics::{OpenGL, GlyphCache};

use crate::interface::Interface;
use crate::creature::Creature;
use crate::physics;
use crate::draw;
use crate::io;
use crate::io::{Sensor, Decision};
use crate::constants;

pub struct World {
    creatures: Vec<Creature>,
}

impl World {
    pub fn add_creature(&mut self, creature: Creature) {
        self.creatures.push(creature);
    }
    pub fn remove_creature(&mut self, index: usize) {
        self.creatures.remove(index);
    }
}

pub struct Game {
    pub gl: GlGraphics,
    pub frames: u32,
    pub elapsed: f64,
    pub world: World,
    pub interface: Interface
}

pub trait GameObject {
    fn update(&mut self, args: &UpdateArgs);
    fn render(&mut self, args: &RenderArgs, context: &graphics::Context, gl: &mut GlGraphics);
    fn render_debug(&mut self, args: &RenderArgs, context: &graphics::Context, gl: &mut GlGraphics);
}

impl Game {

    pub fn new(opengl: OpenGL) -> Self {
        return Game {
            gl: GlGraphics::new(opengl),
            frames: 0,
            elapsed: 0.0,
            //glyphs: glyphs,
            interface: Interface::new(),
            world: World {
                creatures: Vec::new()
            }
        }
    }

    pub fn render(&mut self, args: &RenderArgs) {

        constants::set_window_width(args.window_size[0] as usize);
        constants::set_window_height(args.window_size[1] as usize);

        self.gl.draw(args.viewport(), |context: graphics::Context, gl| {
            graphics::clear(graphics::color::BLACK, gl);

            // if self.interface.creature_stats.is_some() {
            //     self.interface.render(args, &context, gl);
            // }

            for creature in self.world.creatures.iter_mut() {
                creature.render(args, &context, gl);
                if constants::DEBUG {
                    creature.render_debug(args, &context, gl);
                }
            }

            if constants::DEBUG {
                let half_w = args.window_size[0] / 2.0;
                let half_h = args.window_size[1] / 2.0;
                let quar_w = half_w / 2.0;
                let quar_h = half_h / 2.0;

                draw::circle(
                    graphics::color::GREEN,
                    1.0,
                    half_w,
                    half_h,
                    &context,
                    gl
                );
                let bbox = graphics::rectangle::rectangle_by_corners(
                    quar_w,
                    quar_h,
                    half_w + quar_w,
                    half_h + quar_h,
                );
                let border = graphics::Rectangle::new_round_border(
                    graphics::color::WHITE,
                    0.5,
                    0.5,
                ); 
                border.draw(
                    bbox,
                    &context.draw_state,
                    context.transform,
                    gl
                );
            }
        });
    }

    pub fn mouse_input(&mut self, point: &[f32; 2]) {
        for creature in self.world.creatures.iter() {
            if physics::geom::contains(point, &creature.get_bounds()) {
                // for gene in creature.brain.iter() {
                //     println!("{:?}", gene.hex());
                // }
            }
        }
    }
    
    pub fn update(&mut self, args: &UpdateArgs) {
        let len = self.world.creatures.len();
        for x in 0..len {
            let mut creature = self.world.creatures.get(x).unwrap().clone();

            let mut inputs: Vec<f32> = Vec::with_capacity(constants::INPUTS_SIZE as usize);
            inputs.push(io::sense(Sensor::Left(&creature)));
            inputs.push(io::sense(Sensor::Right(&creature)));
            inputs.push(io::sense(Sensor::Up(&creature)));
            inputs.push(io::sense(Sensor::Down(&creature)));

            let (outputs, decision) = creature.brain.compute(&inputs);
            
            let mut dirx = 0.0;
            let mut diry = 0.0;

            match io::decide(decision) {
                Decision::MoveLeft => { dirx = -1.0; },
                Decision::MoveRight => { dirx = 1.0; },
                Decision::MoveUp => { diry = -1.0; },
                Decision::MoveDown => { diry = 1.0; },
                Decision::Sprint => { creature.state.sprint = true; },
                Decision::Walk => { creature.state.sprint = false; },
                Decision::Unknown => {}
            }

            let speed = creature.get_speed();

            creature.state.velocity.0 = speed;
            creature.state.velocity.1 = speed;
  
            let mut dx: f32 = creature.state.velocity.0 * args.dt as f32;
            let mut dy: f32 = creature.state.velocity.1 * args.dt as f32;

            creature.apply_fatigue(0.2);
            if creature.state.stamina > 0.0 {
                dx *= dirx;
                dy *= diry;
            }
    
            creature.state.position.0 += dx;
            creature.state.position.1 += dy;

            for y in 0..len {
                let mut other = self.world.creatures.get_mut(y).unwrap();

                if other.id == creature.id {
                    continue;
                }

                let col = physics::geom::sat_collision(
                    &creature.get_bounds(), &other.get_bounds(),
                );
                if col[0] == 1.0 {
                    creature.state.position.0 += col[1];
                    creature.state.position.1 += col[2];
                    other.state.position.0 -= col[1];
                    other.state.position.1 -= col[2];
                    physics::geom::elastic_collision(&mut creature, &mut other, &0.1);
                }
            }

            physics::geom::boundary_collide(
                &(constants::get_window_width() as f32),
                &(constants::get_window_height() as f32),
                &mut creature
            );

            self.world.creatures[x] = creature;
        }
    }
}