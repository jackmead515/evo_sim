use crate::creature::Creature;
use crate::physics::vec;
use crate::physics::math;

/// Tests if point is contained within a rectangle
pub fn contains(point: &[f32; 2], rect: &[f32; 4]) -> bool {
    let x1 = rect[0];
    let y1 = rect[1];
    let x2 = x1 + rect[2];
    let y2 = y1 + rect[3];

    if point[0] > x1 && point[0] < x2 && point[1] > y1 && point[1] < y2 {
        return true;
    }

    return false;
}

/// Tests if two rectangles are intersecting
pub fn intersecting(rect1: &[f32; 4], rect2: &[f32; 4]) -> bool {
    return rect1[0] < rect2[0] + rect2[2]
        && rect1[0] + rect1[2] > rect2[0]
        && rect1[1] < rect2[1] + rect2[3]
        && rect1[1] + rect1[3] > rect2[1];
}

pub fn boundary_collide(width: &f32, height: &f32, creature: &mut Creature) {
    let bounds = creature.get_bounds();
    if bounds[0] < 0.0 {
        creature.state.position.0 = 0.0;
    }
    if bounds[1] < 0.0 {
        creature.state.position.1 = 0.0;
    }
    if bounds[0] + bounds[2] > *width {
        creature.state.position.0 = width - bounds[2];
    }
    if bounds[1] + bounds[3] > *height {
        creature.state.position.1 = height - bounds[3];
    }
}

pub fn rect_points(rect: &[f32; 4]) -> [[f32; 2]; 4] {
    let mut points: [[f32; 2]; 4] = [[0.0, 0.0], [0.0, 0.0], [0.0, 0.0], [0.0, 0.0]];
    points[0] = [rect[0], rect[1]];
    points[1] = [rect[0], rect[1] + rect[3]];
    points[2] = [rect[0] + rect[2], rect[1] + rect[3]];
    points[3] = [rect[0] + rect[2], rect[1]];
    return points;
}

pub fn rect_edges(points: &[[f32; 2]; 4]) -> [[f32; 2]; 4] {
    let mut edges: [[f32; 2]; 4] = [[0.0, 0.0], [0.0, 0.0], [0.0, 0.0], [0.0, 0.0]];
    edges[0] = vec::subtract(&points[0], &points[1]);
    edges[1] = vec::subtract(&points[1], &points[2]);
    edges[2] = vec::subtract(&points[2], &points[3]);
    edges[3] = vec::subtract(&points[3], &points[0]);
    return edges;
}

pub fn rect_center(rect: &[f32; 4]) -> [f32; 2] {
    return [(rect[0] + rect[2]) / 2.0, (rect[1] + rect[3]) / 2.0];
}

pub fn project(axis: &[f32; 2], points: &[[f32; 2]; 4]) -> [f32; 2] {
    let dt = vec::dot(axis, &points[0]);
    let mut min = dt;
    let mut max = dt;
    for point in points {
        let d = vec::dot(axis, point);
        if d < min {
            min = d;
        } else if d > max {
            max = d;
        }
    }
    return [min, max];
}

pub fn sat_collision(rect1: &[f32; 4], rect2: &[f32; 4]) -> [f32; 3] {
    let mut min_overlap = f32::MAX;
    let mut min_axis = [0.0, 0.0];
    let ps1 = rect_points(rect1);
    let ps2 = rect_points(rect2);
    let ed1 = rect_edges(&ps1);
    let ed2 = rect_edges(&ps2);
    for edge in &ed1 {
        let axis = vec::normalize(&vec::normal(edge));
        let mm1 = project(&axis, &ps1);
        let mm2 = project(&axis, &ps2);
        let overlap = math::max(
            &0.0,
            &(math::min(&mm1[1], &mm2[1]) - math::max(&mm1[0], &mm2[0])),
        );
        if overlap == 0.0 {
            return [0.0, 0.0, 0.0];
        } else if overlap < min_overlap {
            min_overlap = overlap;
            min_axis = axis;
        }
    }
    for edge in &ed2 {
        let axis = vec::normalize(&vec::normal(edge));
        let mm1 = project(&axis, &ps1);
        let mm2 = project(&axis, &ps2);
        let overlap = math::max(
            &0.0,
            &(math::min(&mm1[1], &mm2[1]) - math::max(&mm1[0], &mm2[0])),
        );
        if overlap == 0.0 {
            return [0.0, 0.0, 0.0];
        } else if overlap < min_overlap {
            min_overlap = overlap;
            min_axis = axis;
        }
    }

    let mtv = [min_axis[0] * min_overlap, min_axis[1] * min_overlap];
    let c1c2 = vec::subtract(&rect_center(rect1), &rect_center(rect2));
    if vec::dot(&min_axis, &c1c2) < 0.0 {
        return [1.0, -mtv[0], -mtv[1]];
    }

    return [1.0, mtv[0], mtv[1]];
}

pub fn elastic_collision(c1: &mut Creature, c2: &mut Creature, damping: &f32) {
    let rect1 = c1.get_bounds();
    let rect2 = c2.get_bounds();

    let dx = rect2[0] - rect1[0];
    let dy = rect2[1] - rect1[1];
    let dr = (dx * dx + dy * dy).sqrt();
  
    let nx = dx / dr; //normal x
    let ny = dy / dr; //normal y
  
    let tx = -ny; //tangent x
    let ty = nx; //tangent y
  
    let dpt1 = c1.state.velocity.0 * tx + c1.state.velocity.1 * ty; //dot product of tangent
    let dpt2 = c2.state.velocity.0 * tx + c2.state.velocity.1 * ty;
  
    let dpn1 = c1.state.velocity.0 * nx + c1.state.velocity.1 * ny; //dot product of normal
    let dpn2 = c2.state.velocity.0 * nx + c2.state.velocity.1 * ny;
  
    let m1 = (dpn1 * (c1.traits.mass - c2.traits.mass) + 2.0 * c2.traits.mass * dpn2) / (c1.traits.mass + c2.traits.mass); //momentum
    let m2 = (dpn2 * (c2.traits.mass - c1.traits.mass) + 2.0 * c1.traits.mass * dpn1) / (c1.traits.mass + c2.traits.mass);
    
    c1.state.velocity.0 = (tx * dpt1 + nx * m1) * damping;
    c1.state.velocity.1 = (ty * dpt1 + ny * m1) * damping;
    c2.state.velocity.0 = (tx * dpt2 + nx * m2) * damping;
    c2.state.velocity.1 = (ty * dpt2 + ny * m2) * damping;
}
