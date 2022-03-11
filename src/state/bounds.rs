use rand::Rng;
use rand::prelude::ThreadRng;
use rand;

use std::collections::{HashSet, HashMap};

use crate::state::{GeneExpression, Evolver};
use crate::state::models::{Point, Block, Bounds};
use crate::state::simulation::Constants;

impl Point {
    pub fn new(x: f32, y: f32) -> Self {
        return Point { x: x, y: y };
    }
}

impl Block {

    pub fn new(x: u32, y: u32, size: f32) -> Self {
        // let nx = x as f32 * size;
        // let ny = y as f32 * size;

        return Block {
            position: Point { x: x as f32, y: y as f32 },
            width: size,
            height: size,
        };
    }

    // Translates this block by the x and y and by the radian rotation
    // and returns a new Block with the updated position
    // pub fn translate(&mut self, x: f32, y: f32, rotation: f32) -> Block {
    //     let sin = rotation.sin(); let cos = rotation.cos();

    //     let x0 = (self.p1.x*cos - self.p1.y*sin) + x;
    //     let y0 = (self.p1.x*sin + self.p1.y*cos) + y;

    //     let x1 = (self.p2.x*cos - self.p2.y*sin) + x;
    //     let y1 = (self.p2.x*sin + self.p2.y*cos) + y;

    //     let x2 = (self.p3.x*cos - self.p3.y*sin) + x;
    //     let y2 = (self.p3.x*sin + self.p3.y*cos) + y;

    //     let x3 = (self.p4.x*cos - self.p4.y*sin) + x;
    //     let y3 = (self.p4.x*sin + self.p4.y*cos) + y;

    //     return Block {
    //         p1: Point { x: x0, y: y0 },
    //         p2: Point { x: x1, y: y1 },
    //         p3: Point { x: x2, y: y2 },
    //         p4: Point { x: x3, y: y3 }
    //     };
    // }
}

impl Bounds {

    pub fn new(constants: &Constants) -> Bounds {
        let mut range = rand::thread_rng();

        let mut used_map: HashMap<String, Coordinate> = HashMap::new();
        let mut avaliable = Vec::new();
        
        // start the selection at 0,0 and add
        // left, right, up, and down to avaliable
        // array. Initialized the map of used
        // coordinates with 0,0
        let node = Coordinate::new(0, 0);
        avaliable.push(node.upper_coordinate());
        avaliable.push(node.lower_coordinate());
        avaliable.push(node.left_coordinate());
        avaliable.push(node.right_coordinate());
        used_map.insert(node.key(), node);

        let mut smallest_x: isize = 0;
        let mut smallest_y: isize = 0;

        for _ in 1..constants.block_amount {

            // randomly select an avaliable rect to add.
            let r = range.gen_range(0, avaliable.len()-1);

            // swap_remove, O(1) time
            let node = avaliable.swap_remove(r);

            // these are the next nodes to add to the avaliable
            // map. But they could already be used!
            let next_nodes = vec![
                node.upper_coordinate(),
                node.lower_coordinate(),
                node.left_coordinate(),
                node.right_coordinate()
            ];

            used_map.insert(node.key(), node);

            // loop through and determine the smallest x and y
            // and add nodes to the avaliable set.
            for next_node in next_nodes {
                if !used_map.contains_key(&next_node.key()) {
                    if next_node.x < smallest_x {
                        smallest_x = next_node.x;
                    }
                    if next_node.y < smallest_y {
                        smallest_y = next_node.y;
                    }
                    avaliable.push(next_node);
                }
            }
        }

        let mut blocks = Vec::with_capacity(used_map.len());
        let mut width: u32 = 0;
        let mut height: u32 = 0;

        // normalize all the nodes back to the origin.
        // some coordinates could be negative and we
        // can't have that.
        for node in used_map.values_mut() {
            node.x += smallest_x.abs();
            node.y += smallest_y.abs();

            let x = node.x as u32;
            let y = node.y as u32;

            if x > width {
                width = x;
            }
            if y > height {
                height = y;
            }

            blocks.push(Block::new(x, y, constants.block_size));
        }
       
        return Bounds {
            blocks: blocks,
            width: width,
            height: height
        }
    }

    

    // Translates this block by the x and y and by the radian rotation
    // and returns a new Block with the updated position
    // pub fn translate(&self, x: f32, y: f32, rotation: f32) -> Bounds {
    //     let mut new_bounds = self.clone();

    //     for block in new_bounds.blocks.iter_mut() {
    //         *block = block.translate(x, y, rotation);
    //     }

    //     return new_bounds;
    // }

}

impl Evolver for Bounds {
    fn evolve(&self, constants: &Constants) -> Bounds {
        let new_bounds = self.clone();
    
        return new_bounds;
    }
}

impl GeneExpression for Bounds {
    fn gene_codes(&self, constants: &Constants) -> Vec<String> {
        return Vec::new();
    }
}

pub struct Coordinate {
    x: isize,
    y: isize
}

impl Coordinate {

    pub fn new(x: isize, y: isize) -> Coordinate {
        return Coordinate {
            x: x,
            y: y
        }
    }

    pub fn key(&self) -> String {
        return format!("{},{}", self.x, self.y);
    }

    pub fn upper_coordinate(&self) -> Coordinate {
        return Coordinate {
            x: self.x,
            y: self.y - 1
        }
    }

    pub fn lower_coordinate(&self) -> Coordinate {
        return Coordinate {
            x: self.x,
            y: self.y + 1
        }
    }

    pub fn left_coordinate(&self) -> Coordinate {
        return Coordinate {
            x: self.x - 1,
            y: self.y
        }
    }

    pub fn right_coordinate(&self) -> Coordinate {
        return Coordinate {
            x: self.x + 1,
            y: self.y
        }
    }

}