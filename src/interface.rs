use opengl_graphics::{GlGraphics};
use piston::input::{RenderArgs, UpdateArgs};

use crate::creature::Creature;
use crate::game::GameObject;

pub struct CreatureStats {
    pub creature_id: Option<u32>,
}

pub struct Interface {
    pub creature_stats: CreatureStats,
}

impl GameObject for Interface {
    fn render(&mut self, args: &RenderArgs, context: &graphics::Context, gl: &mut GlGraphics) {
        let [width, height] = args.window_size;
        let dims = [width - 110.0, 10.0, 100.0, 100.0];
        graphics::rectangle(graphics::color::WHITE, dims, context.transform, gl);

        // graphics::text(
        //     graphics::color::BLACK,
        //     16,
        //     "hello world",
        //     cache: &mut C,
        //     transform: math::Matrix2d,
        //     g: &mut G
        // );
    }

    fn render_debug(&mut self, args: &RenderArgs, context: &graphics::Context, gl: &mut GlGraphics) {
    
        
    }

    fn update(&mut self, args: &UpdateArgs) {

    }
}

impl Interface {
    pub fn new() -> Self {
        return Interface {
            creature_stats: CreatureStats {
                creature_id: Option::None,
            }
        }
    }
}
