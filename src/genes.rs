use opengl_graphics;
use rand::Rng;
use rand;

#[derive(Clone, Copy)]
pub struct Traits {
    pub size: f32,
    pub sprint_speed: f32,
    pub walk_speed: f32,
    pub endurance: f32,
    pub mass: f32,
    pub color: [f32; 4],
}

#[derive(Clone, Copy)]
pub struct Gene {
    pub id: u8,
    pub input: u8,
    pub node: u8,
    pub output: u8,
}

impl Traits {

    pub fn random() -> Self {
        let mut range = rand::thread_rng();

        return Traits {
            size: range.gen_range(5f32, 15f32),
            sprint_speed: range.gen_range(20f32, 40f32),
            walk_speed: range.gen_range(5f32, 10f32),
            endurance: range.gen_range(10f32, 20f32),
            mass: range.gen_range(10f32, 20f32),
            color: graphics::color::RED,
        }
    }

}

impl Gene {

    pub fn hex(&self) -> String {
        let mut hex: String = String::new();
        hex.push_str(&format!("{:x}", self.id)[..]);
        hex.push_str(&format!("{:x}", self.input)[..]);
        hex.push_str(&format!("{:x}", self.node)[..]);
        hex.push_str(&format!("{:x}", self.output)[..]);
        return hex;
    }

}