use std::{collections::HashMap, time::Instant};

// personal functions
//use crate::utils::grid2d;
//use crate::utils::pause;
//use crate::utils::math;

pub fn main() {
    println!("-- Advent of Code - Day 02 - Part 1 --");
    let now = Instant::now();

    //let input = include_str!("../assets/day02_input_demo1.txt");
    let input = include_str!("../assets/day02_input.txt");

    println!("La réponse est {}", get_answer(input));

    let elapsed1 = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed1);
}

//

fn get_answer(input: &str) -> usize {
    let mut nbr_2 = 0;
    let mut nbr_3 = 0;
    let mut product=0;
    input
        .lines()
        .for_each(|line| {
            let mut counts = HashMap::new();
            for ch in line.chars() {
                *counts.entry(ch).or_insert(0) += 1;
            }
            let x:Vec<usize>  = counts.into_values().collect();
            if x.contains(&2) {nbr_2 += 1;}
            if x.contains(&3) {nbr_3 += 1;}
            product = nbr_2*nbr_3;
        });
        product
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_total() {
        assert_eq!(
            get_answer(include_str!("../assets/day02_input_demo1.txt")),
            12
        );
        assert_eq!(get_answer(include_str!("../assets/day02_input.txt")), 6696);
    }
}
