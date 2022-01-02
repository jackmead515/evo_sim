use rand::Rng;
use rand::prelude::ThreadRng;
use rand;

use std::collections::HashSet;

use crate::state::models::{Point, Block, Bounds};

/*

[
    [0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 1, 0, 0],
    [0, 1, 1, 1, 1, 0, 0],
    [0, 0, 1, 1, 1, 0, 0],
    [0, 0, 1, 1, 1, 1, 0],
    [0, 0, 1, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0],
]

pos = [0, 0]

for row in matrix {
    for col in row {
        if matrix[row][col] == 1 {
            box = [
                [col * size       ,        row * size]
                [col * size + size,        row * size]
                [col * size + size, row * size + size]
                [col * size       , row * size + size]
            ]
        }
    }
}
*/

fn create_placement_matrix(amount: usize) -> Vec<Vec<u8>> {
    let mut dims = amount * 2;
    if dims % 2 == 0 {
        dims += 1;
    }

    let mut matrix: Vec<Vec<u8>> = Vec::with_capacity(dims);
    for rows in 0..dims {
        let mut row: Vec<u8> = Vec::with_capacity(dims);
        for cols in 0..dims {
            row.push(0);
        }
        matrix.push(row);
    }

    return matrix;
}

/// up    -> 0
/// down  -> 1
/// left  -> 2
/// right -> 3
fn get_direction_pool(last_direction: Option<usize>) -> Vec<usize> {
    if last_direction.is_some() {
        let mut directions = Vec::new();
        let lastd = last_direction.unwrap();
        for d in 0..4 {
            if d == 0 && lastd == 1 {
                continue;
            } else if d == 1 && lastd == 0 {
                continue;
            } else if d == 2 && lastd == 3 {
                continue;
            } else if d == 3 && lastd == 2 {
                continue;
            }
            directions.push(d);
        }

        return directions;
    }

    return vec![0, 1, 2, 3];
}

impl Point {
    pub fn new(x: f32, y: f32) -> Self {
        return Point { x: x, y: y };
    }
}

impl Block {
    pub fn from_coords(x: usize, y: usize, size: f32) -> Self {
        let nx = x as f32 * size;
        let ny = y as f32 * size;
        let nys = ny + size;
        let nxs = nx + size;

        return Block (
            Point { x: nx, y: ny },
            Point { x: nx, y: nys },
            Point { x: nxs, y: nys },
            Point { x: nxs, y: ny },
        );
    }

    pub fn new(size: f32) -> Self {
        return Block (
            Point { x: 0.0, y: 0.0 },
            Point { x: 0.0, y: size },
            Point { x: size, y: size },
            Point { x: size, y: 0.0 },
        );
    }

    pub fn new_up(block: &Block, size: f32) -> Self {
        return Block (
            Point { x: block.0.x, y: block.0.y - size },
            Point { x: block.1.x, y: block.1.y - size },
            Point { x: block.2.x, y: block.2.y - size },
            Point { x: block.3.x, y: block.3.y - size },
        );
    }

    pub fn new_down(block: &Block, size: f32) -> Self {
        return Block (
            Point { x: block.0.x, y: block.0.y + size },
            Point { x: block.1.x, y: block.1.y + size },
            Point { x: block.2.x, y: block.2.y + size },
            Point { x: block.3.x, y: block.3.y + size },
        )
    }

    pub fn new_left(block: &Block, size: f32) -> Self {
        return Block (
            Point { x: block.0.x - size, y: block.0.y },
            Point { x: block.1.x - size, y: block.1.y },
            Point { x: block.2.x - size, y: block.2.y },
            Point { x: block.3.x - size, y: block.3.y },
        )
    }

    pub fn new_right(block: &Block, size: f32) -> Self {
        return Block (
            Point { x: block.0.x + size, y: block.0.y },
            Point { x: block.1.x + size, y: block.1.y },
            Point { x: block.2.x + size, y: block.2.y },
            Point { x: block.3.x + size, y: block.3.y },
        )
    }

    pub fn to_verts(&self) -> Vec<Vec<f32>> {
        return vec![
            vec![self.0.x, self.0.y],
            vec![self.1.x, self.1.y],
            vec![self.2.x, self.2.y],
            vec![self.3.x, self.3.y],
        ]
    }
}

pub fn random_bounds(size: f32, amount: usize) -> Bounds {
    let mut matrix = create_placement_matrix(amount);

    let mid = (matrix.len() - 1) / 2;
    let mut row = mid;
    let mut col = mid;

    matrix[row][col] = 1;

    let mut range = rand::thread_rng();
    let mut blocks = Vec::with_capacity(amount);
    blocks.push(Block::from_coords(row, col, size));

    let mut created = 1;

    while created < amount {
        let mut nrow = row as isize;
        let mut ncol = col as isize;

        let r = range.gen_range(0, 3);

        match r {
            0 => { nrow += 1; }
            1 => { nrow -= 1; }
            2 => { ncol += 1; }
            3 => { ncol -= 1; }
            _ => panic!("invalid random generated: {}", r)
        }

        if (nrow < 0 || matrix.len() as isize <= nrow) || 
        (ncol < 0 || matrix[nrow as usize].len() as isize <= ncol) {
            continue;
        } else if matrix[nrow as usize][ncol as usize] == 1 {
            row = nrow as usize;
            col = ncol as usize;
            continue;
        }

        let block = Block::from_coords(nrow as usize, ncol as usize, size);
        blocks.push(block);

        matrix[nrow as usize][ncol as usize] = 1;
        row = nrow as usize;
        col = ncol as usize;
        created += 1;
    }
        
    // for _ in 0..amount-2 {
    //     let last_block = &blocks[blocks.len()-1];

    //     let pool = get_direction_pool(last_direction);
    //     let direction = range.gen_range(0, pool.len());
        
    //     match direction {
    //         0 => {
    //             blocks.push(Block::new_up(last_block, size));
    //         }
    //         1 => {
    //             blocks.push(Block::new_down(last_block, size));
    //         }
    //         2 => {
    //             blocks.push(Block::new_left(last_block, size));
    //         }
    //         3 => {
    //             blocks.push(Block::new_right(last_block, size));
    //         }
    //         _ => panic!("invalid direction: {}", direction)
    //     }

    //     last_direction = Option::Some(direction);
    // }

    // let mut block_set: HashSet<Block> = HashSet::new();

    // for block in blocks {
    //     block_set.insert(block);
    // }
    
    // println!("{} blocks generated", block_set.len());

    return Bounds {
        blocks: blocks
    }
}