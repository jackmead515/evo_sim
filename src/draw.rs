use opengl_graphics::{GlGraphics};

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