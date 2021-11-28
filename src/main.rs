extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;
extern crate rand;
extern crate nalgebra;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL, GlyphCache, TextureSettings};
use piston::event_loop::{EventSettings, EventLoop, Events};
use piston::input::{RenderArgs, MouseCursorEvent, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;
use rand::Rng;

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

    for id in 0..100 {
        let mut creature = Creature::new(
            id,
            constants::BRAIN_SIZE,
            constants::INPUTS_SIZE,
            constants::OUTPUTS_SIZE,
        );
        let rand_x: f32 = rando.gen_range(50.0f32, window_width as f32 - 50.0);
        let rand_y: f32 = rando.gen_range(50.0f32, window_height as f32 - 50.0);
        creature.set_center(rand_x, rand_y);
        creature.traits.sprint_speed = 100.0;
        creature.traits.walk_speed = 50.0;
        game.world.add_creature(creature);
    }

    let mut events = Events::new(EventSettings::new());

    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.mouse_cursor_args() {
            let mouse_loc = [args[0] as f32, args[1] as f32];
            game.mouse_input(&mouse_loc);
        }
        if let Some(args) = e.render_args() {
            game.frames += 1;
            game.render(&args);
        }
        if let Some(args) = e.update_args() {
            game.elapsed += args.dt;
            game.update(&args);
            if game.elapsed > 1.0 {
                let fps = (game.frames as f64) / game.elapsed;
                println!("FPS: {}", fps);
                game.frames = 0;
                game.elapsed = 0.0;
            }
        }
    }
}
