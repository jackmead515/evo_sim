use opengl_graphics::{GlGraphics};
use rand::Rng;
use rand;

pub fn random_color() -> [f32; 4] {
    let mut range = rand::thread_rng();
    let r = range.gen_range(0usize, 255usize);
    let g = range.gen_range(0usize, 255usize);
    let b = range.gen_range(0usize, 255usize);
    return [r as f32, g as f32, b as f32, 1.0];
}

pub fn circle(
    color: [f32; 4],
    radius: f64,
    x: f64,
    y: f64,
    context: &graphics::Context,
    gl: &mut GlGraphics
) {
    let square = graphics::rectangle::square(x, y, 0.0);
    graphics::circle_arc(color, radius, 0.0, 8.0, square, context.transform, gl);
}