use rand::Rng;
use rand::prelude::ThreadRng;
use rand;

use std::collections::HashSet;

use crate::state::models::{Point, Block, Bounds, Constants};

fn get_bounds_width(matrix: &Vec<Vec<u8>>) -> usize {
    let mut minc = matrix.len();
    let mut maxc = 0;

    for row in 0..matrix.len() {
        for col in 0..matrix[row].len() {
            if matrix[row][col] == 1 && col < minc {
                minc = col
            }
            if matrix[row][col] == 1 && col > maxc {
                maxc = col
            }
        }
    }

    return (maxc - minc) + 1;
}

fn get_bounds_height(matrix: &Vec<Vec<u8>>) -> usize {
    let mut height = 0;

    for row in 0..matrix.len() {
        for col in 0..matrix[row].len() {
            if matrix[row][col] == 1 {
                height += 1;
                break;
            }
        }
    }

    return height;
}

fn create_placement_matrix(amount: usize) -> Vec<Vec<u8>> {
    let mut dims = amount * 2;
    if dims % 2 == 0 {
        dims += 1;
    }

    let mut matrix: Vec<Vec<u8>> = Vec::with_capacity(dims);
    for _rows in 0..dims {
        let mut row: Vec<u8> = Vec::with_capacity(dims);
        for _cols in 0..dims {
            row.push(0);
        }
        matrix.push(row);
    }

    return matrix;
}

impl Point {
    pub fn new(x: f32, y: f32) -> Self {
        return Point(x, y);
    }
}

impl Block {
    pub fn from_coords(x: usize, y: usize, size: f32) -> Self {
        let nx = x as f32 * size;
        let ny = y as f32 * size;
        let nys = ny + size;
        let nxs = nx + size;

        return Block (
            Point(nx, ny),
            Point(nx, nys),
            Point(nxs, nys),
            Point(nxs, ny)
        );
    }

    pub fn to_verts(&self) -> Vec<Vec<f32>> {
        return vec![
            vec![self.0.0, self.0.1],
            vec![self.1.0, self.1.1],
            vec![self.2.0, self.2.1],
            vec![self.3.0, self.3.1],
        ]
    }

    /// Translates this block by the x and y and by the radian rotation
    /// and returns a new Block with the updated position
    pub fn translate(&mut self, x: f32, y: f32, rotation: f32) -> Block {
        let sin = rotation.sin(); let cos = rotation.cos();

        let x0 = (self.0.0*cos - self.0.1*sin) + x;
        let y0 = (self.0.0*sin + self.0.1*cos) + y;

        let x1 = (self.1.0*cos - self.1.1*sin) + x;
        let y1 = (self.1.0*sin + self.1.1*cos) + y;

        let x2 = (self.2.0*cos - self.2.1*sin) + x;
        let y2 = (self.2.0*sin + self.2.1*cos) + y;

        let x3 = (self.3.0*cos - self.3.1*sin) + x;
        let y3 = (self.3.0*sin + self.3.1*cos) + y;

        return Block (
            Point(x0, y0),
            Point(x1, y1),
            Point(x2, y2),
            Point(x3, y3)
        );
    }
}

impl Bounds {

    pub fn new(constants: &Constants) -> Bounds {
        let mut matrix = create_placement_matrix(constants.block_amount);

        let mid = (matrix.len() - 1) / 2;
        let mut row = mid;
        let mut col = mid;

        matrix[row][col] = 1;

        let mut range = rand::thread_rng();
        let mut blocks = Vec::with_capacity(constants.block_amount);
        blocks.push(Block::from_coords(row, col, constants.block_size));

        let mut created = 1;

        while created < constants.block_amount {
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

            if (nrow < 0 || matrix.len() as isize <= nrow) || (ncol < 0 || matrix[nrow as usize].len() as isize <= ncol) {
                continue;
            }

            let urow = nrow as usize;
            let ucol = ncol as usize;
            
            if matrix[urow][ucol] == 1 {
                row = urow;
                col = ucol;
                continue;
            }

            let block = Block::from_coords(ucol, urow, constants.block_size);
            blocks.push(block);

            matrix[urow][ucol] = 1;
            row = urow;
            col = ucol;
            created += 1;
        }

        let width = get_bounds_width(&matrix);
        let height = get_bounds_height(&matrix);

        return Bounds {
            blocks: blocks,
            width: width,
            height: height
        }
    }

    pub fn evolve(&self, constants: &Constants) -> Bounds {
        let new_bounds = self.clone();
        
        return new_bounds;
    }

    /// Translates this block by the x and y and by the radian rotation
    /// and returns a new Block with the updated position
    pub fn translate(&self, x: f32, y: f32, rotation: f32) -> Bounds {
        let mut new_bounds = self.clone();

        for block in new_bounds.blocks.iter_mut() {
            *block = block.translate(x, y, rotation);
        }

        return new_bounds;
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

        if (nrow < 0 || matrix.len() as isize <= nrow) || (ncol < 0 || matrix[nrow as usize].len() as isize <= ncol) {
            continue;
        }

        let urow = nrow as usize;
        let ucol = ncol as usize;
        
        if matrix[urow][ucol] == 1 {
            row = urow;
            col = ucol;
            continue;
        }

        let block = Block::from_coords(ucol, urow, size);
        blocks.push(block);

        matrix[urow][ucol] = 1;
        row = urow;
        col = ucol;
        created += 1;
    }

    let width = get_bounds_width(&matrix);
    let height = get_bounds_height(&matrix);

    return Bounds {
        blocks: blocks,
        width: width,
        height: height
    }
}