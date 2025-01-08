use std::{collections::HashMap, time::Instant};

// personal functions
//use crate::utils::grid2d;
//use crate::utils::pause;
//use crate::utils::math;

pub fn main() {
    println!("-- Advent of Code - Day 04 - Part 1 --");
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
    let mut guards : HashMap<u32,Vec<(u32,u32)>> = HashMap::new();
    let mut prev_guard : u32 = 0;
    let mut prev_asleep:u32 = 0;
    input.lines().for_each(|line| {
        if line.chars().collect::<Vec<char>>().contains(&'#') {
            let guard_id: u32 = line.split_whitespace().collect::<Vec<&str>>()[3][1..].parse::<u32>().unwrap();
            // println!("guard {}",guard_id);
            prev_guard = guard_id;
        } else if line.split_whitespace().collect::<Vec<&str>>().contains(&"asleep") {
            prev_asleep = line.split([' ',':',']']).collect::<Vec<&str>>()[2].parse::<u32>().unwrap();
            // println!("asleep at {}",prev_asleep);            
        } else if line.split_whitespace().collect::<Vec<&str>>().contains(&"wakes") {
            let wakeup = line.split([' ',':',']']).collect::<Vec<&str>>()[2].parse::<u32>().unwrap();
            // println!("wakeup at {}",wakeup);
            if guards.contains_key(&prev_guard) {
                let mut old_vec = guards.get(&prev_guard).unwrap().clone();
                old_vec.push((prev_asleep,wakeup));
                guards.insert(prev_guard, old_vec.clone());
            } else {
                let v = vec![(prev_asleep,wakeup)];
                guards.insert(prev_guard, v.clone());
            }
        }
    });
    // println!("--");
    // println!("{:?}",guards);
    // println!("--");
    let mut lazy = 0;
    let mut time_sleep =0;
    let mut big = 0;
    guards.iter().for_each( |guard| {
        time_sleep = 0;
        guard.1.iter().for_each(|(sleep,wake)| {
            time_sleep += wake-sleep;
        });
        if time_sleep > big {
            big= time_sleep;
            lazy = *guard.0;
        }
    });
    println!("lazier guard is {} who slept {} min",lazy,big);
    println!("{:?}",guards.get(&lazy));
    let mut minuts : HashMap<u32,u32> = HashMap::new();
    guards.get(&lazy).unwrap().iter().for_each(|(sleep,wake)| {
        for t in *sleep..*wake {            
            let current = minuts.get(&t).unwrap_or(&0);
            minuts.insert(t, current+1);
        }
    });
    println!("{:?}",minuts);
    let max = minuts.iter().max_by_key(|&(_, v)| v).unwrap();
    println!("{} x {}",lazy,max.0);
    lazy* *max.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_total() {
        assert_eq!(
            get_answer(include_str!("../assets/day04_input_demo1.txt")),
            0
        );
        assert_eq!(get_answer(include_str!("../assets/day04_input.txt")), 0);
    }
}
