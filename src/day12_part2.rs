use std::{cmp::Ordering, collections::HashMap, time::Instant};

// personal functions
//use crate::utils::grid2d;
//use crate::utils::pause;
//use crate::utils::math;

pub fn main() {
    println!("-- Advent of Code - Day 12 - Part 2 --");
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
    let mut offset = 5; // number of points added before string
    let mut str_pts = String::new();
    for _ in 0..offset {
        str_pts.push('.');
    }
    state = format!("{}{}{}", str_pts, state, str_pts);

    let cycle = 100;
    for _gen in 1..=cycle {
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
        manage_start_end(&mut state, &mut offset);
        // if gen%100 == 0 {
        // println!("{:2}: {}", gen, state);
        // println!("offset is {}",offset);
        // }
    }

    // after 100 cycles, the result doesn't change. Only the offset does.
    // it can be calculated as :
    let cycle = 50000000000_i64;
    let offset = -((((cycle / 100) - 1) * 100) + 69);
    // println!("calc offset is {}",offset);

    let mut result: isize = 0;
    for (i, c) in state.chars().enumerate() {
        if c == '#' {
            result += i as isize - offset as isize;
        }
    }

    Some(result)
}

fn manage_start_end(state: &mut String, offset: &mut i64) {
    // we try to alway have 5 leading points
    let start_pos = state.chars().position(|x| x == '#').unwrap();
    let delta = 5 - start_pos as i64;
    *offset += delta;
    if delta.cmp(&0) == Ordering::Greater {
        let pts = ".....".to_string();
        let zz = pts.chars().take(delta as usize).collect::<String>();
        *state = format!("{}{}", zz, state);
        //
    } else if delta.cmp(&0) == Ordering::Less {
        *state = state.chars().skip(-delta as usize).collect::<String>();
    }

    let end_pos = state.chars().rev().position(|x| x == '#').unwrap();
    let delta = 5 - end_pos as i64;
    if delta.cmp(&0) == Ordering::Greater {
        let pts = ".....".to_string();
        let zz = pts.chars().take(delta as usize).collect::<String>();
        *state = format!("{}{}", state, zz);
        //
    } else if delta.cmp(&0) == Ordering::Less {
        *state = state[..(state.len() as i64 + delta) as usize].to_string();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_start_end() {
        let mut str = ".......#".to_string();
        let mut offset = 10;
        manage_start_end(&mut str, &mut offset);
        assert_eq!(str, ".....#.....".to_string());
        assert_eq!(offset, 8);
    }
    #[test]
    fn test_total() {
        // assert_eq!(
        //     get_answer(include_str!("../assets/day12_input_demo1.txt")),
        //     Some(325)
        // );
        assert_eq!(
            get_answer(include_str!("../assets/day12_input.txt")),
            Some(1250000000991)
        );
    }
}
