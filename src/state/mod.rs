pub mod models;
pub mod bounds;
pub mod brain;
pub mod traits;
pub mod creature;
pub mod simulation;

use crate::state::simulation::Constants;

pub trait GeneExpression {
    fn gene_codes(&self, constants: &Constants) -> Vec<u8>;

    fn ascii_codes(&self, codes: &Vec<u8>) -> Vec<String> {
        // 65 - 90 == A - Z
        let min_char = 65.0;
        let max_char = 90.0;

        let mut genes: Vec<String> = Vec::new();
        let mut index = 0;
        let gene_code_size = 4;

        while index < codes.len() {

            // partition up the weights into partitions of 4
            let mut slice: Vec<u8> = Vec::with_capacity(gene_code_size);
            for i in index..index+4 {
                if i < codes.len() {
                    slice.push(codes[i]);
                }
            }
            
            // append 65, or A, if there is no more norms left
            while slice.len() != gene_code_size {
                slice.push(min_char as u8);
            }

            // convert numbers to String in ASCII
            let code = String::from_utf8(slice).unwrap();
            genes.push(code);

            index += gene_code_size;
        }

        return genes;
    }
    
    fn gene_rgba_color(&self, codes: &Vec<u8>) -> Vec<f32> {
        let mut color: Vec<f32> = Vec::new();

        let min_num = 0.0;
        let max_num = 255.0;
        let min_char = 65.0;
        let max_char = 90.0;
        let partition_size = (codes.len() as f32 / 3.0).ceil() as usize;
        let mut index = 0;

        while index < codes.len() {
            let mut sum = 0.0;
            let mut total = 0.0;

            for i in index..index+partition_size {
                if i < codes.len() {
                    total += 1.0;
                    sum += codes[i] as f32;
                }
            }

            let avg = sum / total;
            let norm = (((avg - min_char) / (max_char - min_char)) * (max_num - min_num)) + min_num;

            color.push(norm);

            index += partition_size;
        }

        color.push(1.0);

        return color;
    }
}

pub trait Evolver {
    fn evolve(&self, constants: &Constants) -> Self;
}