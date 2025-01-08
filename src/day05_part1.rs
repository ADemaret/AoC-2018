use std::time::Instant;

// personal functions
//use crate::utils::grid2d;
//use crate::utils::pause;
//use crate::utils::math;

pub fn main() {
    println!("-- Advent of Code - Day 05 - Part 1 --");
    let now = Instant::now();

    // let input = include_str!("../assets/day05_input_demo1.txt");
    let input = include_str!("../assets/day05_input.txt");

    if let Some(answer) = get_answer(input) {
        println!("The answer is : {}", answer);
    } else {
        println!("No answer found");
    }
    let elapsed1 = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed1);
}

// 9705 too high

fn get_answer(input: &str) -> Option<usize> {
    let mut v: Vec<char> = input.chars().collect();
    loop {
        // println!("{}", v.iter().collect::<String>() );
        let mut to_remove = None;
        for i in 0..(v.len() - 1) {
            // println!("{},{} => {},{}", v[i], v[i + 1], v[i] as u32, v[i] as u32);
            if (v[i] as u32).abs_diff(v[i + 1] as u32) == 32 {            
                to_remove = Some(i);
                break;
            }
        }

        if to_remove.is_none() {
            break;
        } else {
            let index = to_remove.unwrap();
            // println!(
            //     "removing {} and {} at pos {}",
            //     v[index],
            //     v[index + 1],
            //     index
            // );
            v.remove(index);
            v.remove(index);
        }
    }
    // println!("{}", v.iter().collect::<String>());
    Some(v.len())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_total() {
        assert_eq!(
            get_answer(include_str!("../assets/day05_input_demo1.txt")),
            Some(10)
        );
        assert_eq!(get_answer(include_str!("../assets/day05_input.txt")), Some(9704));
    }
}
