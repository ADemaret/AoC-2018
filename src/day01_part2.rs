use std::time::Instant;

// personal functions
//use crate::utils::grid2d;
//use crate::utils::pause;
//use crate::utils::math;

pub fn main() {
    println!("-- Advent of Code - Day 01 - Part 2 --");
    let now = Instant::now();

    //let input = include_str!("../assets/day01_input_demo1.txt");
    let input = include_str!("../assets/day01_input.txt");

    println!("La réponse est {}", get_answer(input));

    let elapsed1 = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed1);
}

//

fn get_answer(input: &str) -> isize {
    let mut reached: Vec<isize> = Vec::new();
    let mut freq: isize = 0;
    let i = input
        .lines()
        .map(|d| d.parse::<isize>().unwrap())
        .collect::<Vec<isize>>();
    loop {
        for x in &i {
            freq += x;
            if !reached.contains(&freq) {
                reached.push(freq)
            } else {
                return freq;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_total() {
        assert_eq!(
            get_answer(include_str!("../assets/day01_input_demo1.txt")),
            2
        );
        assert_eq!(
            get_answer(include_str!("../assets/day01_input.txt")),
            124645
        );
    }
}
