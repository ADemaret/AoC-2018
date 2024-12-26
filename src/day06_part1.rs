use std::{collections::HashMap, time::Instant};

// personal functions
//use crate::utils::grid2d;
// use crate::utils::pause;
//use crate::utils::math;

const MAX_SIZE: usize = 350;

pub fn main() {
    println!("-- Advent of Code - Day 06 - Part 1 --");
    let now = Instant::now();

    // let input = include_str!("../assets/day06_input_demo1.txt");
    let input = include_str!("../assets/day06_input.txt");

    if let Some(answer) = get_answer(input) {
        println!("The answer is : {}", answer);
    } else {
        println!("No answer found");
    }
    let elapsed1 = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed1);
}

//
#[derive(PartialEq)]
enum Cell {
    Common,
    Id(usize),
}

fn get_answer(input: &str) -> Option<usize> {
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
    let mut grid: HashMap<(usize, usize), Cell> = HashMap::new();
    let mut index = 0;
    for l in 0..MAX_SIZE {
        for c in 0..MAX_SIZE {
            if coords.contains(&(l, c)) {
                grid.insert((c, l), Cell::Id(index)); // inverted !!
                index += 1;
            }
        }
    }
    // print_grid(&grid);

    // set zones
    for l in 0..MAX_SIZE {
        for c in 0..MAX_SIZE {
            let mut manhattan = Vec::new();
            for (index, pt) in coords.iter().enumerate() {
                let dist: usize = get_manhattan_distance((l, c), *pt);
                manhattan.push((dist, index + 1));
            }
            let min_dist = manhattan.iter().min().unwrap();
            if manhattan.iter().filter(|(d, _)| *d == min_dist.0).count() > 1 {
                grid.insert((c, l), Cell::Common);
            } else {
                grid.insert((c, l), Cell::Id(min_dist.1));
            }
        }
    }
    // print_grid(&grid);

    // count zone sizes
    let mut zones = HashMap::new();
    for l in 0..MAX_SIZE {
        for c in 0..MAX_SIZE {
            let cell = grid.get(&(l, c)).unwrap();
            match cell {
                Cell::Id(value) => {
                    if l == 0 || l == MAX_SIZE - 1 || c == 0 || c == MAX_SIZE - 1 {
                        // zone is infinite
                        *zones.entry(*value).or_insert(0) += MAX_SIZE * MAX_SIZE;
                    } else {
                        *zones.entry(*value).or_insert(0) += 1;
                    }
                }
                Cell::Common => {}
            }
        }
    }
    // println!("zones : {:?}", zones);
    let v = zones
        .iter()
        .map(|(&z, &nb)| (z, nb))        
        .filter(|(_, nb)| *nb < (MAX_SIZE * MAX_SIZE)).max_by_key(|x| x.1).unwrap();
    Some(v.1)
}

fn get_manhattan_distance((l1, c1): (usize, usize), (l2, c2): (usize, usize)) -> usize {
    l1.abs_diff(l2) + c1.abs_diff(c2)
}

// fn print_grid(grid: &HashMap<(usize, usize), Cell>) {
//     for l in 0..MAX_SIZE {
//         for c in 0..MAX_SIZE {
//             let tup = (l, c);
//             if let Some(v) = grid.get(&tup) {
//                 match v {
//                     Cell::Id(v) => {
//                         print!("{}", v)
//                     }
//                     Cell::Common => {
//                         print!(".")
//                     }
//                 }
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
            get_answer(include_str!("../assets/day06_input_demo1.txt")),
            Some(17)
        );
        assert_eq!(
            get_answer(include_str!("../assets/day06_input.txt")),
            Some(4398)
        );
    }
}
