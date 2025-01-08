use std::time::Instant;

// personal functions
//use crate::utils::grid2d;
//use crate::utils::pause;
//use crate::utils::math;

pub fn main() {
    println!("-- Advent of Code - Day XX - Part 1 --");
    let now = Instant::now();

    let input = include_str!("../assets/dayXX_input_demo1.txt");
    // let input = include_str!("../assets/dayXX_input.txt");

    if let Some(answer) = get_answer(input) {
        println!("The answer is : {}", answer);
    } else {
        println!("No answer found");
    }
    let elapsed1 = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed1);
}

//

fn get_answer(input: &str) -> Option<usize> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_total() {
        assert_eq!(get_answer(include_str!("../assets/dayXX_input_demo1.txt")), None);
        assert_eq!(get_answer(include_str!("../assets/dayXX_input.txt")), None);
    }
}
