
use crate::creature::Creature;
use crate::constants;

pub enum Sensor<'a> {
    Left(&'a Creature),
    Right(&'a Creature),    
    Up(&'a Creature),
    Down(&'a Creature),
}

pub enum Decision {
    MoveLeft,
    MoveRight,
    MoveUp,
    MoveDown,
    Sprint,
    Walk,
    Nothing,
    Unknown,
}

pub fn decide(decision: u8) -> Decision {
    return match decision {
        0 => Decision::MoveLeft,
        1 => Decision::MoveRight,
        2 => Decision::MoveUp,
        3 => Decision::MoveDown,
        4 => Decision::Sprint,
        5 => Decision::Walk,
        6 => Decision::Nothing,
        _ => Decision::Unknown,
    }
}

pub fn sense(sensor: Sensor) -> f32 {
    return match sensor {
        Sensor::Left(creature) => sense_left(creature),
        Sensor::Right(creature) => sense_right(creature),
        Sensor::Up(creature) => sense_up(creature),
        Sensor::Down(creature) => sense_down(creature)
    }
}

pub fn sense_up(creature: &Creature) -> f32 {
    let mut distance_to_up = creature.state.position.1;
    distance_to_up /= constants::get_window_height();
    return distance_to_up as f32;
}

pub fn sense_down(creature: &Creature) -> f32 {
    let window_height = constants::get_window_height();
    let mut distance_to_down = window_height - creature.state.position.1;
    distance_to_down /= window_height;
    return distance_to_down as f32;
}

pub fn sense_right(creature: &Creature) -> f32 {
    let mut distance_to_right = creature.state.position.0;
    distance_to_right /= constants::get_window_width() as isize;
    return distance_to_right as f32;
}

pub fn sense_left(creature: &Creature) -> f32 {
    let window_width = constants::get_window_width() as isize;
    let mut distance_to_left = window_width - creature.state.position.0;
    distance_to_left /= window_width;
    return distance_to_left as f32;
}