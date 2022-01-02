use opengl_graphics::{GlGraphics};
use piston::input::{RenderArgs, UpdateArgs};
use rand;

use crate::game::{GameObject, World};
use crate::genes::{Traits};
use crate::brain::Brain;
use crate::draw;

#[derive(Clone, Copy, Debug)]
pub struct State {
    pub position: (isize, isize),
    pub direction: (isize, isize),
    //pub velocity: (f32, f32),
    pub collision: (isize, isize),
    pub sprint: bool,
    pub health: f32,
    pub stamina: f32
}

#[derive(Clone)]
pub struct Creature {
    pub id: u128,
    pub state: State,
    pub brain: Brain,
    pub traits: Traits
}

impl Creature {

    pub fn new(id: u128, brain_size: u8, inputs_size: u8, output_size: u8) -> Self {
        return Creature {
            id: id,
            brain: Brain::new_random(brain_size, inputs_size, output_size),
            traits: Traits::random(),
            state: State {
                //velocity: (0.0, 0.0),
                collision: (0, 0),
                position: (0, 0),
                direction: (0, 0),
                sprint: false,
                stamina: 100.0,
                health: 100.0,
            },
        };
    }

    pub fn get_speed(&self) -> isize {
        if self.state.sprint && self.state.stamina > 0.0 {
            return self.traits.sprint_speed;
        } else {
            return self.traits.walk_speed;
        }
    }

    pub fn set_position(&mut self, x: isize, y: isize) {
        self.state.position.0 = x;
        self.state.position.1 = y;
    }

    pub fn set_center(&mut self, x: isize, y: isize) {
        let size = self.traits.size;
        self.state.position.0 = x - (size / 2) as isize;
        self.state.position.1 = y - (size / 2) as isize;
    }

    pub fn get_center(&self) -> (isize, isize) {
        let half_size = (self.traits.size / 2) as isize;
        return (self.state.position.0 + half_size, self.state.position.1 + half_size);
    }

    pub fn get_bounds(&self) -> [isize; 4] {
        return [
            self.state.position.0,
            self.state.position.1,
            self.traits.size,
            self.traits.size,
        ];
    }

    pub fn apply_fatigue(&mut self, factor: f32, delta: f32) {
        if self.state.sprint {
            let final_stamina = self.state.stamina - (factor * delta * self.traits.endurance);
            if final_stamina < 0.0 {
                self.state.stamina = 0.0;
            } else {
                self.state.stamina = final_stamina;
            }
        }
    }

}

impl GameObject for Creature {

    fn render(&mut self, args: &RenderArgs, context: &graphics::Context, gl: &mut GlGraphics) {
        let border = graphics::Rectangle::new_round_border(
            graphics::color::WHITE,
            0.5,
            0.5,
        ); 

        let area = graphics::Rectangle::new_round(
            graphics::color::RED,
            1.0
        );

        let bbox = graphics::rectangle::square(
            self.state.position.0 as f64,
            self.state.position.1 as f64,
            self.traits.size as f64
        );

        border.draw(
            bbox,
            &context.draw_state,
            context.transform,
            gl
        );
        area.draw(
            bbox,
            &context.draw_state, 
            context.transform,
            gl
        );
    }

    fn render_debug(&mut self, args: &RenderArgs, context: &graphics::Context, gl: &mut GlGraphics) {
        let center = self.get_center();
        draw::circle(
            graphics::color::GREEN,
            1.0,
            center.0 as f64,
            center.1 as f64,
            context,
            gl
        )
    }

    fn update(&mut self, args: &UpdateArgs) {}
    
}