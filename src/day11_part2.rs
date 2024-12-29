use std::time::Instant;

// personal functions
//use crate::utils::grid2d;
//use crate::utils::pause;
//use crate::utils::math;

const GRID_SIZE: usize = 300;

pub fn main() {
    println!("-- Advent of Code - Day 11 - Part 2 --");
    let now = Instant::now();

    let input = include_str!("../assets/day11_input.txt");

    if let Some(answer) = get_answer(input) {
        println!("The answer is : {}", answer);
    } else {
        println!("No answer found");
    }
    let elapsed1 = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed1);
}

//

fn get_answer(input: &str) -> Option<String> {
    let input_val = input.parse::<i32>().unwrap();

    let grid = set_grid(input_val);

    // for x in 1..GRID_SIZE {
    //     for y in 1..GRID_SIZE {
    //         print!("{:3}",grid[x][y]);
    //     }
    //     println!()
    // }

    let (x, y, z) = get_max(grid);

    Some(format!("{x},{y},{z}"))
}

fn get_max(grid: [[i32; GRID_SIZE]; GRID_SIZE]) -> (usize, usize, usize) {
    let mut result = (0, 0, 0);
    let mut v_max = 0;
    for size in 2..250 {
        println!("test size {} - max is {}", size, v_max);
        for x in 1..=(GRID_SIZE - size) {
            for y in 1..=(GRID_SIZE - size) {
                let mut v = 0;
                for xx in 0..size {
                    for yy in 0..size {
                        if (x + xx) < 300 && y + yy < 300 {
                            v += grid[x + xx][y + yy];
                        }
                    }
                }
                if v > v_max {
                    v_max = v;
                    result = (x, y, size);
                }
            }
        }
    }

    result
}

fn set_grid(input_val: i32) -> [[i32; GRID_SIZE]; GRID_SIZE] {
    let mut grid = [[0; GRID_SIZE]; GRID_SIZE];

    for x in 1..GRID_SIZE {
        for y in 1..GRID_SIZE {
            let rack_id = x as i32 + 10;
            let mut val = (rack_id * y as i32 + input_val) * rack_id;
            let sub = val % 100;
            val = ((val - sub) / 100) % 10;
            val -= 5;
            grid[x][y] = val;
        }
    }
    grid
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_grid() {
        let grid = set_grid(8);
        assert_eq!(grid[3][5], 4);
        let grid = set_grid(57);
        assert_eq!(grid[122][79], -5);
        let grid = set_grid(39);
        assert_eq!(grid[217][196], 0);
        let grid = set_grid(71);
        assert_eq!(grid[101][153], 4);
    }

    #[test]
    fn test_get_max() {
        let grid = set_grid(18);
        assert_eq!(get_max(grid), (90, 269, 16));
        let grid = set_grid(42);
        assert_eq!(get_max(grid), (232, 251, 12));
    }

    #[test]
    fn test_total() {
        assert_eq!(
            get_answer(include_str!("../assets/day11_input.txt")),
            Some("231,65,14".to_string())
        );
    }
}
