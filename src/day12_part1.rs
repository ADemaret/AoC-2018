use std::{collections::HashMap, time::Instant};

// personal functions
//use crate::utils::grid2d;
//use crate::utils::pause;
//use crate::utils::math;

pub fn main() {
    println!("-- Advent of Code - Day 12 - Part 1 --");
    let now = Instant::now();

    // let input = include_str!("../assets/day12_input_demo1.txt");
    let input = include_str!("../assets/day12_input.txt");

    if let Some(answer) = get_answer(input) {
        println!("The answer is : {}", answer);
    } else {
        println!("No answer found");
    }
    let elapsed1 = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed1);
}

//

fn get_answer(input: &str) -> Option<isize> {
    // get input
    let (p1, p2) = input.split_once("\n\n").unwrap();
    let mut state = p1.split_once(": ").unwrap().1.to_string();
    let notes = p2
        .lines()
        .map(|line| line.split_once(" => ").unwrap())
        .collect::<HashMap<&str, &str>>();

    // add "..." at start and end
    let offset = 5;
    let mut str_pts = String::new();
    for _ in 0..offset {
        str_pts.push('.');
    }
    state = format!("{}{}{}", str_pts, state, str_pts);

    for gen in 1..=20 {
        let mut new_state = String::new();
        for i in 0..state.len() - 3 {
            let sub = state.chars().skip(i).take(5).collect::<String>();
            if let Some(n) = notes.get(sub.as_str()) {
                new_state.push_str(n);
            } else {
                new_state.push('.');
            }
        }
        state = format!("..{}..", new_state);
        println!("{:2}: {}", gen, state);
    }

    let mut result: isize = 0;
    for (i, c) in state.chars().enumerate() {
        if c == '#' {
            result += i as isize - offset as isize;
        }
    }

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_total() {
        assert_eq!(
            get_answer(include_str!("../assets/day12_input_demo1.txt")),
            Some(325)
        );
        assert_eq!(get_answer(include_str!("../assets/day12_input.txt")), Some(1917));
    }
}
