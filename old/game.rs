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
    pub mouse_pos: [isize; 2],
    pub frames: u32,
    pub updates: u32,
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
            updates: 0,
            mouse_pos: [0, 0],
            interface: Interface::new(),
            world: World {
                creatures: Vec::new()
            }
        }
    }

    pub fn render(&mut self, args: &RenderArgs) {
        constants::set_window_width(args.window_size[0] as isize);
        constants::set_window_height(args.window_size[1] as isize);

        self.gl.draw(args.viewport(), |context: graphics::Context, gl| {
            graphics::clear(graphics::color::BLACK, gl);

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

    pub fn mouse_input(&mut self) {
        // for creature in self.world.creatures.iter() {
        //     if physics::geom::contains(&self.mouse_pos, &creature.get_bounds()) {
        //         //println!("creature {} traits: {:?}", creature.id, creature.traits);
        //     }
        // }
    }

    pub fn mouse_press(&mut self) {
        for creature in self.world.creatures.iter() {
            if physics::geom::contains(&self.mouse_pos, &creature.get_bounds()) {
                println!("creature {} traits: {:?}, state: {:?}", creature.id, creature.traits, creature.state);
            }
        }
    }
    
    pub fn update(&mut self, args: &UpdateArgs) {
        let delta = args.dt as f32;
        let creature_length = self.world.creatures.len();

        for creature_index in 0..creature_length {
            let mut creature = self.world.creatures.get(creature_index).unwrap().clone();

            let mut inputs: Vec<f32> = Vec::with_capacity(constants::INPUTS_SIZE as usize);
            inputs.push(io::sense(Sensor::Left(&creature)));
            inputs.push(io::sense(Sensor::Right(&creature)));
            inputs.push(io::sense(Sensor::Up(&creature)));
            inputs.push(io::sense(Sensor::Down(&creature)));

            let (outputs, decision) = creature.brain.compute(&inputs);

            match io::decide(decision) {
                Decision::MoveLeft => { creature.state.direction.0 = -1; },
                Decision::MoveRight => { creature.state.direction.0 = 1; },
                Decision::MoveUp => { creature.state.direction.1 = -1; },
                Decision::MoveDown => { creature.state.direction.1 = 1; },
                Decision::Sprint => { creature.state.sprint = true; },
                Decision::Walk => { creature.state.sprint = false; },
                Decision::Nothing => {
                    creature.state.direction.0 = 0;
                    creature.state.direction.1 = 0;
                }
                Decision::Unknown => {}
            }

            creature.apply_fatigue(0.2, delta);
            let movement_speed = creature.get_speed();

            'outer: for _ in 0..movement_speed {
                creature.state.position.0 += creature.state.direction.0;
                creature.state.position.1 += creature.state.direction.1;

                let mut collision = false;

                for other_creature in self.world.creatures.iter_mut() {
                    if other_creature.id == creature.id {
                        continue;
                    }
    
                    if physics::geom::intersecting(&creature.get_bounds(), &other_creature.get_bounds()) {
                        creature.state.position.0 -= creature.state.direction.0;
                        creature.state.position.1 -= creature.state.direction.1;
                        collision = true;
                    }
                }

                if collision {
                    break;
                }
            }

            physics::geom::world_collide(
                &(constants::get_window_width()),
                &(constants::get_window_height()),
                &mut creature
            );

            self.world.creatures[creature_index] = creature;
        }
    }
}