

pub fn speed_from_mass(mass: f32) -> f32 {
    return 8000.0 * mass.powf(-2.0);
}

pub fn endurance_from_mass(mass: f32) -> f32 {
    return 100000.0 * mass.powf(-2.0);
}

pub fn mass_from_size(size: isize) -> f32 {
    return 5.0 * size as f32;
}