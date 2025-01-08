use std::time::Instant;

// personal functions
//use crate::utils::grid2d;
//use crate::utils::pause;
//use crate::utils::math;

pub fn main() {
    println!("-- Advent of Code - Day 05 - Part 2 --");
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
    let mut shortest = 100000 ;
    for ch in 'a'..='z' {
        let mut v: Vec<char> = input.chars().collect();
        shortest = v.len().min(shortest);
        print!("removing {} from {}", ch, v.iter().collect::<String>());
        let mut i = 0;
        loop {
            if i >= v.len() - 1 {
                break;
            }
            while v[i].to_ascii_lowercase() == ch {
                v.remove(i);
                i = i.saturating_sub(1);
                if i == 0 || i >= v.len() -1 {
                    break;
                }
            }
            while (v[i] as u32).abs_diff(v[i + 1] as u32) == 32 {
                v.remove(i);
                v.remove(i);
                // println!("{}", v.iter().collect::<String>());
                i = i.saturating_sub(1);
                if i == v.len() - 1 {
                    break;
                }
            }
            i += 1;
            if i == v.len() - 1 {
                break;
            }
        }
        println!(" => {}", v.iter().collect::<String>());
        if v.len() < shortest {
            println!("{} is shortest than {}",v.len(), shortest);
            shortest = v.len();
        }
    }
    Some(shortest)
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
        assert_eq!(
            get_answer(include_str!("../assets/day05_input.txt")),
            Some(9704)
        );
    }
}
