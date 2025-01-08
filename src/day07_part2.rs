use std::{collections::HashMap, time::Instant};

// personal functions
//use crate::utils::grid2d;
//use crate::utils::pause;
//use crate::utils::math;

pub fn main() {
    println!("-- Advent of Code - Day 07 - Part 2 --");
    let now = Instant::now();

    // let input = include_str!("../assets/day07_input_demo1.txt");
    // let workers = 2;
    // let offset = 0;
    let input = include_str!("../assets/day07_input.txt");
    let workers = 5;
    let offset = 60;

    if let Some(answer) = get_answer(input, workers, offset) {
        println!("The answer is : {}", answer);
    } else {
        println!("No answer found");
    }
    let elapsed1 = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed1);
}

//

fn get_answer(input: &str, nbr_workers: usize, time_offset: usize) -> Option<usize> {
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
        states.insert(a, usize::MAX);
        states.insert(b, usize::MAX);
    });

    // links
    let mut workers = vec![0; nbr_workers];

    let mut time = 0;
    loop {
        // print!("-- time {} - ", time);

        let mut poss = Vec::new();
        // list possible steps
        'out: for i in &instr {
            // if not yet set
            if *states.get(i.0).unwrap() == usize::MAX {
                if i.1.is_empty() {
                    poss.push(i.0);
                } else {
                    for n in i.1 {
                        if let Some(b) = states.get(n) {
                            if *b > time {
                                continue 'out;
                            }
                        }
                    }
                    poss.push(i.0);
                }
            }
        }

        if !poss.is_empty() {
            poss.sort();
            let mut index_to_process = 0;
            for w in workers.iter_mut() {
                if *w <= time {
                    if poss.len() <= index_to_process {
                        break;
                    }
                    let delay =
                        time + time_offset + (*poss[index_to_process] as usize - 'A' as usize + 1);

                    *w = delay;
                    states.insert(*poss[index_to_process], delay);
                    index_to_process += 1;
                }
            }
        }

        if states
            .iter()
            .filter(|&x| *x.1 > time && *x.1 < usize::MAX)
            .count()
            == 0
        {
            // println!("all done !");
            break;
        }
        // print!("processed : ");
        // for x in &states {
        //     if *x.1 > time && *x.1 < usize::MAX {
        //         print!("{} ", x.0);
        //     }
        // }
        // println!();

        time += 1;
    }

    Some(time)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_total() {
        assert_eq!(
            get_answer(include_str!("../assets/day07_input_demo1.txt"), 2, 0),
            Some(15)
        );
        assert_eq!(
            get_answer(include_str!("../assets/day07_input.txt"), 5, 60),
            Some(946)
        );
    }
}
