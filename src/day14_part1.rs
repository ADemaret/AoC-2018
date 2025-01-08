use std::time::Instant;

// personal functions
//use crate::utils::grid2d;
//use crate::utils::pause;
//use crate::utils::math;

pub fn main() {
    println!("-- Advent of Code - Day 14 - Part 1 --");
    let now = Instant::now();

    let input = include_str!("../assets/day14_input.txt");

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
    let i = input.parse::<usize>().unwrap();
    let mut scores = vec![3, 7];
    let mut current = (0, 1);
    // print_scores(&scores, current);

    loop {
        let new_score = (scores[current.0] + scores[current.1]).to_string();
        new_score.chars().for_each(|c| {
            scores.push(c as usize - '0' as usize);
        });
        current.0 = (current.0 + scores[current.0] + 1) % scores.len();
        current.1 = (current.1 + scores[current.1] + 1) % scores.len();
        // print_scores(&scores, current);

        if scores.len() >= (2 + 2 * i).max(i + 10) {
            return Some(
                scores
                    .iter()
                    .skip(i)
                    .take(10)
                    .map(|&num| num.to_string())
                    .collect::<String>(),
            );
        }
    }
}

// fn print_scores(scores: &[usize], current: (usize, usize)) {
//     for (i, s) in scores.iter().enumerate() {
//         match i {
//             val if val == current.0 => print!("("),
//             val if val == current.1 => print!("["),
//             _ => print!(" "),
//         }
//         print!("{}", *s);
//         match i {
//             val if val == current.0 => print!(")"),
//             val if val == current.1 => print!("]"),
//             _ => print!(" "),
//         }
//     }
//     println!();
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_total() {
        assert_eq!(get_answer("9"), Some("5158916779".to_string()));
        assert_eq!(get_answer("5"), Some("0124515891".to_string()));
        assert_eq!(get_answer("18"), Some("9251071085".to_string()));
        assert_eq!(get_answer("2018"), Some("5941429882".to_string()));
        assert_eq!(get_answer(include_str!("../assets/day14_input.txt")), Some("1411383621".to_string()));
    }
}
