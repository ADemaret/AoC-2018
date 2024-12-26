use std::time::Instant;

// personal functions
//use crate::utils::grid2d;
//use crate::utils::pause;
//use crate::utils::math;

pub fn main() {
    // return;
    println!("-- Advent of Code - Day 03 - Part 2 --");
    let now = Instant::now();

    // let input = include_str!("../assets/day03_input_demo1.txt");
    let input = include_str!("../assets/day03_input.txt");

    println!("La réponse est {}", get_answer(input));

    let elapsed1 = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed1);
}

// 107989 too low
// 111326

fn get_answer(input: &str) -> usize {
    let mut used: Vec<(usize, usize)> = Vec::new();
    let mut multi_used: Vec<(usize, usize)> = Vec::new();
    input.lines().for_each(|line| {
        let parts = line
            .split(['#', '@', ' ', ',', ':', 'x'])
            .filter(|s| !s.is_empty())
            .map(|d| d.parse::<usize>().unwrap())
            .collect::<Vec<_>>();
        // println!("{:?}", parts);
        // println!(
        //     "les cases utilisées par le tissu {} ont les coordonnées...",
        //     parts[0]
        // );
        for x in (parts[2] + 1)..(parts[2] + 1 + parts[4]) {
            for y in (parts[1] + 1)..(parts[1] + 1 + parts[3]) {
                // println!("({x},{y})");
                if !used.contains(&(x, y)) {
                    used.push((x, y))
                } else if !multi_used.contains(&(x, y)) {
                    multi_used.push((x, y))
                }
            }
        }
    });
    for line in input.lines() {
        let parts = line
            .split(['#', '@', ' ', ',', ':', 'x'])
            .filter(|s| !s.is_empty())
            .map(|d| d.parse::<usize>().unwrap())
            .collect::<Vec<_>>();
        let mut used = false;
        for x in (parts[2] + 1)..(parts[2] + 1 + parts[4]) {
            for y in (parts[1] + 1)..(parts[1] + 1 + parts[3]) {
                if multi_used.contains(&(x, y)) {
                    used = true;
                    break;
                }
            }
        }
        if !used {
            return parts[0];
        }
    };
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_total() {
        assert_eq!(
            get_answer(include_str!("../assets/day03_input_demo1.txt")),
            3
        );
        assert_eq!(get_answer(include_str!("../assets/day03_input.txt")), 1019);
    }
}
