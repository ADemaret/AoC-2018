use std::{collections::HashMap, time::Instant};

// personal functions
//use crate::utils::grid2d;
// use crate::utils::pause;
//use crate::utils::math;

const MAX_SIZE: usize = 350;
// const MAX_SIZE: usize = 10;

pub fn main() {
    println!("-- Advent of Code - Day 06 - Part 2 --");
    let now = Instant::now();

    // let input = include_str!("../assets/day06_input_demo1.txt");
    let input = include_str!("../assets/day06_input.txt");

    if let Some(answer) = get_answer(input,10000) {
        println!("The answer is : {}", answer);
    } else {
        println!("No answer found");
    }
    let elapsed1 = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed1);
}

//
fn get_answer(input: &str, max:usize) -> Option<usize> {
    // read input
    let mut coords = input
        .lines()
        .map(|line| {
            line.split_once(", ")
                .map(|(l, c)| (l.parse::<usize>().unwrap(), c.parse::<usize>().unwrap()))
                .unwrap()
        })
        .collect::<Vec<_>>();
    coords.sort();
    // println!("{:?}", coords);

    // set a grid with input points
    let mut grid: HashMap<(usize, usize), char> = HashMap::new();
    for l in 0..MAX_SIZE {
        for c in 0..MAX_SIZE {
            if coords.contains(&(l, c)) {
                grid.insert((c, l), 'X'); // inverted !!
            }
        }
    }
    // print_grid(&grid);

    // set zones
    for l in 0..MAX_SIZE {
        for c in 0..MAX_SIZE {
            let mut tot_manhattan = 0;
            for (_index, pt) in coords.iter().enumerate() {
                let dist: usize = get_manhattan_distance((l, c), *pt);
                tot_manhattan += dist;
            }
            if tot_manhattan < max {
                grid.insert((c, l), '#');
            } else {
                grid.insert((c, l), '.');
            }
        }
    }
    // print_grid(&grid);

    // count zone sizes
    let mut result = 0;
    // let mut zones = HashMap::new();
    for l in 0..MAX_SIZE {
        for c in 0..MAX_SIZE {
            let char = grid.get(&(l, c)).unwrap();
            if *char == '#' {
                result += 1;
            }
        }
    }    
    Some(result)
}

fn get_manhattan_distance((l1, c1): (usize, usize), (l2, c2): (usize, usize)) -> usize {
    l1.abs_diff(l2) + c1.abs_diff(c2)
}

// fn print_grid(grid: &HashMap<(usize, usize), char>) {
//     for l in 0..MAX_SIZE {
//         for c in 0..MAX_SIZE {
//             let tup = (l, c);
//             if let Some(v) = grid.get(&tup) {
//                 print!("{}", v)
//             } else {
//                 print!("_");
//             }
//         }
//         println!();
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_total() {
        assert_eq!(
            get_answer(include_str!("../assets/day06_input_demo1.txt"),32),
            Some(16)
        );
        assert_eq!(
            get_answer(include_str!("../assets/day06_input.txt"),10000),
            Some(39560)
        );
    }
}
