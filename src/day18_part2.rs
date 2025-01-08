use std::time::Instant;

use crate::utils::grid2d::Grid2D;

// personal functions
//use crate::utils::grid2d;
// use crate::utils::pause;
//use crate::utils::math;

pub fn main() {
    println!("-- Advent of Code - Day 18 - Part 2 --");
    let now = Instant::now();

    // let input = include_str!("../assets/day18_input_demo1.txt");
    let input = include_str!("../assets/day18_input.txt");

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
    let grid = Grid2D::new(input);

    // let iter = 10;
    // loops(&mut grid, iter);

    let mut cycle = 0;
    let max_cycles = 1000000000;
    let mut found = false;
    let mut tortue = grid.clone();
    let mut lievre = grid.clone();
    // in the worst case (less than a cycle or no cycle), we do all the iterations
    for i in 1..=max_cycles {
        loops(&mut tortue, 1);
        loops(&mut lievre, 2);
        // compare
        if tortue == lievre {
            cycle = i;
            found = true;
            // println!("cycle found !! : {}", cycle);
            break;
        }
    }

    let _iterations_done = if found {
        let remaining_iterations = max_cycles % cycle;
        // println!("remaining iterations : {}", remaining_iterations);
        loops(&mut tortue, remaining_iterations);
        cycle + remaining_iterations
    } else {
        max_cycles
    };
    let result = tortue.count_occurences('#') * tortue.count_occurences('|');

    // println!("The value after {max_cycles} iterations is {result}. It needed {iterations_done} iterations");

    // grid.print();

    Some(result)
}

fn loops(grid: &mut Grid2D, iter: usize) {
    let mut grid2 = grid.clone();

    for _ in 0..iter {
        for l in 0..grid.max_l {
            for c in 0..grid.max_c {
                let adj = grid.get_adjacents(l, c);
                match grid.get_at((l, c)) {
                    '.' => {
                        if adj.iter().filter(|&x| x.2 == '|').collect::<Vec<_>>().len() >= 3 {
                            grid2.set_at((l, c), '|');
                        };
                    }
                    '|' => {
                        if adj.iter().filter(|&x| x.2 == '#').collect::<Vec<_>>().len() >= 3 {
                            grid2.set_at((l, c), '#');
                        };
                    }
                    '#' => {
                        if !adj
                            .iter()
                            .filter(|&x| x.2 == '#')
                            .collect::<Vec<_>>()
                            .is_empty()
                            && !adj
                                .iter()
                                .filter(|&x| x.2 == '|')
                                .collect::<Vec<_>>()
                                .is_empty()
                        {
                            grid2.set_at((l, c), '#');
                        } else {
                            grid2.set_at((l, c), '.');
                        };
                    }
                    _ => panic!(),
                }
            }
        }
        *grid = grid2.clone();
        // grid.print();
        // pause::pause();
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_total() {
        // assert_eq!(
        //     get_answer(include_str!("../assets/day18_input_demo1.txt")),
        //     Some(1147)
        // );
        assert_eq!(
            get_answer(include_str!("../assets/day18_input.txt")),
            Some(210824)
        );
    }
}
