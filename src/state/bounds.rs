use rand::Rng;
use rand::prelude::ThreadRng;
use rand;

use std::collections::{HashSet, HashMap};

use crate::state::{GeneExpression, Evolver};
use crate::state::models::*;

// 65 - 90 == A - Z
const min_char: f32 = 65.0;
const max_char: f32 = 90.0;

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

        for _ in 1..constants.initial_block_amount {

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
        let mut width: f32 = 0.0;
        let mut height: f32 = 0.0;

        // normalize all the nodes back to the origin.
        // some coordinates could be negative and we
        // can't have that.
        for node in used_map.values_mut() {
            node.x += smallest_x.abs();
            node.y += smallest_y.abs();

            let x = (node.x - 1) as f32;
            let y = (node.y - 1) as f32;

            if x > width {
                width = x;
            }
            if y > height {
                height = y;
            }

            blocks.push(Point { x: x, y: y });
        }
        
        // add the block size because the x,y describes
        // the top-left corner position
        // width += constants.block_size;
        // height += constants.block_size;

        return Bounds {
            blocks: blocks,
            dimensions: Dimension {
                width: width,
                height: height
            }
        }
    }
}

impl Evolver for Bounds {
    fn evolve(&self, constants: &Constants) -> Bounds {
        let new_bounds = self.clone();
        let mut range = rand::thread_rng();

        let amount_chance = range.gen_range(0.0, 1.0);
        if amount_chance >= constants.block_amount_evolve_chance {

        }

        let arange_chance = range.gen_range(0.0, 1.0);
        if arange_chance >= constants.block_arrange_evolve_chance {

        }

        return new_bounds;
    }
}

impl GeneExpression for Bounds {
    fn gene_codes(&self, constants: &Constants) -> Vec<u8> {
        let mut weight_norms: Vec<u8> = Vec::new();
        let max_block_pos = (self.dimensions.width * self.dimensions.height) as f32;

        for block in self.blocks.iter() {
            let x = block.x;
            let y = block.y;
            let norm = (((x+y) / max_block_pos) * (max_char - min_char)) + min_char;

            // should be a safe cast norm to u8 because 
            // ascii is between 65 - 90. u8 is between 0 and 255.
            weight_norms.push(norm as u8);
        }

        return weight_norms;
    }
}

struct Coordinate {
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