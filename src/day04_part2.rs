use std::{collections::HashMap, time::Instant};

// personal functions
//use crate::utils::grid2d;
//use crate::utils::pause;
//use crate::utils::math;

pub fn main() {
    println!("-- Advent of Code - Day 04 - Part 2 --");
    let now = Instant::now();

    // let input = include_str!("../assets/day04_input_demo1.txt");
    let input = include_str!("../assets/day04_input.txt");

    println!("La réponse est {}", get_answer(input));

    let elapsed1 = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed1);
}

//

fn get_answer(input: &str) -> u32 {
    // println!("{}",input);
    let mut guards: HashMap<u32, Vec<(u32, u32)>> = HashMap::new();
    let mut prev_guard: u32 = 0;
    let mut prev_asleep: u32 = 0;
    input.lines().for_each(|line| {
        if line.chars().collect::<Vec<char>>().contains(&'#') {
            let guard_id: u32 = line.split_whitespace().collect::<Vec<&str>>()[3][1..]
                .parse::<u32>()
                .unwrap();
            // println!("guard {}",guard_id);
            prev_guard = guard_id;
        } else if line
            .split_whitespace()
            .collect::<Vec<&str>>()
            .contains(&"asleep")
        {
            prev_asleep = line.split([' ', ':', ']']).collect::<Vec<&str>>()[2]
                .parse::<u32>()
                .unwrap();
            // println!("asleep at {}",prev_asleep);
        } else if line
            .split_whitespace()
            .collect::<Vec<&str>>()
            .contains(&"wakes")
        {
            let wakeup = line.split([' ', ':', ']']).collect::<Vec<&str>>()[2]
                .parse::<u32>()
                .unwrap();
            // println!("wakeup at {}",wakeup);
            if guards.contains_key(&prev_guard) {
                let mut old_vec = guards.get(&prev_guard).unwrap().clone();
                old_vec.push((prev_asleep, wakeup));
                guards.insert(prev_guard, old_vec.clone());
            } else {
                let v = vec![(prev_asleep, wakeup)];
                guards.insert(prev_guard, v.clone());
            }
        }
    });
    // println!("--");
    // println!("{:?}", guards);
    // println!("--");

    let mut minuts: HashMap<u32, u32> = HashMap::new();
    let mut max_id = 0;
    let mut max_minuts = 0;
    let mut max_times :u32 = 0;
    guards.iter().for_each(|guard| {
        guard.1.iter().for_each(|(sleep, wake)| {
            for t in *sleep..*wake {
                let current = minuts.get(&t).unwrap_or(&0);
                minuts.insert(t, current + 1);
            }
        });
        // println!("minuts for guard {} : {:?}", guard.0, minuts);
        let max = minuts.iter().max_by_key(|&(_, v)| v).unwrap();
        // println!("max is {} at {}",max.1, max.0);
        if max.1 > &max_times {
            max_times = *max.1;
            max_id = *guard.0;
            max_minuts = *max.0;
        }

        minuts.clear();
    });
    // println!("max max is {} at {} for guard {}",max_times, max_minuts, max_id);
    max_minuts * max_id
    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_total() {
        assert_eq!(
            get_answer(include_str!("../assets/day04_input_demo1.txt")),
            4455
        );
        assert_eq!(get_answer(include_str!("../assets/day04_input.txt")), 14920);
    }
}
