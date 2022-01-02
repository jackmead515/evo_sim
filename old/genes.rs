use opengl_graphics;
use rand::Rng;
use rand;

use crate::physics::phy;
use crate::draw::random_color;

#[derive(Clone, Copy, Debug)]
pub struct Traits {
    pub size: isize,
    pub sprint_speed: isize,
    pub walk_speed: isize,
    pub endurance: f32,
    pub mass: f32,
    pub color: [f32; 4],
}

#[derive(Clone, Copy)]
pub struct Genes {
    pub id: u8,
    pub input: u8,
    pub node: u8,
    pub output: u8,
}

impl Traits {

    pub fn random() -> Self {
        let mut range = rand::thread_rng();

        let size = range.gen_range(5isize, 15isize);
        let mass = phy::mass_from_size(size);
        let walk_speed = phy::speed_from_mass(mass);
        let endurance = phy::endurance_from_mass(mass);
        let sprint_speed = walk_speed * 2.0;

        let traits = Traits {
            size: size,
            sprint_speed: sprint_speed as isize,
            walk_speed: walk_speed as isize,
            endurance: endurance,
            mass: mass,
            color: random_color(),
        };

        println!("traits: {:?}", traits);

        return traits;
    }

}

impl Genes {

    pub fn hex(&self) -> String {
        let mut hex: String = String::new();
        hex.push_str(&format!("{:x}", self.id)[..]);
        hex.push_str(&format!("{:x}", self.input)[..]);
        hex.push_str(&format!("{:x}", self.node)[..]);
        hex.push_str(&format!("{:x}", self.output)[..]);
        return hex;
    }

}