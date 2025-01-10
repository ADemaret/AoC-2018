use std::{collections::HashMap, time::Instant};

use crate::utils::grid2d::Grid2D;

// personal functions
//use crate::utils::grid2d;
//use crate::utils::pause;
//use crate::utils::math;

pub fn main() {
    println!("-- Advent of Code - Day 22 - Part 1 --");
    let now = Instant::now();

    // let input = include_str!("../assets/day22_input_demo1.txt");
    let input = include_str!("../assets/day22_input.txt");

    if let Some(answer) = get_answer(input) {
        println!("The answer is : {}", answer);
    } else {
        println!("No answer found");
    }
    let elapsed1 = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed1);
}

//

fn get_answer(input: &str) -> Option<usize> {
    let (l1, l2) = input.split_once("\n").unwrap();
    let (_, d) = l1.split_once(" ").unwrap();
    let depth = d.parse::<usize>().unwrap();
    let v = l2
        .split([' ', ','])
        .filter_map(|x| x.parse::<usize>().ok())
        .collect::<Vec<_>>();
    let target = (v[0], v[1]);
    // println!("depth:{}, target:{:?}", depth, target);

    // add map offset after target
    let grid_size = (target.1 + 1, target.0 + 1);
    let mut grid = Grid2D::new_empty(grid_size.0, grid_size.1, ' ');

    let mut gi = HashMap::new(); // geologic index
    let mut el = HashMap::new(); // erosion level

    gi.insert((0, 0), 0);
    el.insert((0, 0), 0);
    gi.insert((target.1, target.0), 0);
    el.insert((target.1, target.0), depth % 20183);

    for x in 1..=grid_size.1 {
        gi.insert((x, 0), x * 16807);
        el.insert((x, 0), (x * 16807 + depth) % 20183);
    }
    for y in 1..=grid_size.0 {
        gi.insert((0, y), y * 48271);
        el.insert((0, y), (y * 48271 + depth) % 20183);
    }
    for x in 0..=grid_size.1 {
        for y in 0..=grid_size.0 {
            // erosion level
            if !el.contains_key(&(x, y)) {
                let el1 = el.get(&(x - 1, y)).unwrap();
                let el2 = el.get(&(x, y - 1)).unwrap();
                gi.entry((x, y)).or_insert_with(|| el1 * el2);
                el.insert((x, y), ((el1 * el2) + depth) % 20183);
            }
        }
    }

    for x in 0..grid_size.1 {
        for y in 0..grid_size.0 {
            match el.get(&(x, y)).unwrap() % 3 {
                0 => grid.set_at((y, x), '.'), // rock
                1 => grid.set_at((y, x), '='), // water
                2 => grid.set_at((y, x), '|'), // tree
                _ => panic!(),
            }
        }
    }
    grid.set_at((0, 0), 'M');
    grid.set_at((target.1, target.0), 'T');

    // grid.print();

    let result = grid.count_occurences('=') + grid.count_occurences('|') * 2;

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_total() {
        assert_eq!(
            get_answer(include_str!("../assets/day22_input_demo1.txt")),
            Some(114)
        );
        assert_eq!(
            get_answer(include_str!("../assets/day22_input.txt")),
            Some(11462)
        );
    }
}
