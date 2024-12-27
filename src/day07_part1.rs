use std::{collections::HashMap, time::Instant};

// personal functions
//use crate::utils::grid2d;
//use crate::utils::pause;
//use crate::utils::math;

pub fn main() {
    println!("-- Advent of Code - Day 07 - Part 1 --");
    let now = Instant::now();

    // let input = include_str!("../assets/day07_input_demo1.txt");
    let input = include_str!("../assets/day07_input.txt");

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
    let mut instr = HashMap::new();
    let mut states = HashMap::new();
    input.lines().for_each(|line| {
        let chunks = line.split_whitespace().collect::<Vec<_>>();
        let a = chunks[1].chars().next().unwrap();
        let b = chunks[7].chars().next().unwrap();
        instr.entry(a).or_insert(Vec::new());
        if !instr.contains_key(&b) {
            instr.insert(b, vec![a]);
        } else {
            let mut v = instr.get(&b).unwrap().clone();
            v.push(a);
            instr.insert(b, v);
        }
        states.insert(a, false);
        states.insert(b, false);
    });

    // links
    // println!("{:?}", instr);
    // println!("{:?}", states);

    let mut result = Vec::new();

    loop {
        // println!("--");
        let mut poss = Vec::new();
        // list possible steps
        'out: for i in &instr {
            // println!("{:?}", i);
            // if not yet set
            if !states.get(i.0).unwrap() {
                if i.1.is_empty() {
                    poss.push(i.0);
                } else {
                    for n in i.1 {
                        if let Some(b) = states.get(n) {
                            if !*b {
                                continue 'out;
                            }
                        }
                    }
                    poss.push(i.0);
                }
            }
        }
        if poss.is_empty() {
            break;
        }
        poss.sort();
        // println!("=> {:?}", poss);
        result.push(*poss[0]);
        states.insert(*poss[0], true);
    }

    Some(result.iter().collect::<String>())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_total() {
        assert_eq!(
            get_answer(include_str!("../assets/day07_input_demo1.txt")),
            Some("CABDFE".to_string())
        );
        assert_eq!(
            get_answer(include_str!("../assets/day07_input.txt")),
            Some("FHICMRTXYDBOAJNPWQGVZUEKLS".to_string())
        );
    }
}
