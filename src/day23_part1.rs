use std::time::Instant;

// personal functions
//use crate::utils::grid2d;
//use crate::utils::pause;
//use crate::utils::math;

pub fn main() {
    println!("-- Advent of Code - Day 23 - Part 1 --");
    let now = Instant::now();

    // let input = include_str!("../assets/day23_input_demo1.txt");
    let input = include_str!("../assets/day23_input.txt");

    if let Some(answer) = get_answer(input) {
        println!("The answer is : {}", answer);
    } else {
        println!("No answer found");
    }
    let elapsed1 = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed1);
}

//

#[derive(Debug, PartialEq, Eq)]
struct Bot {
    x: isize,
    y: isize,
    z: isize,
    r: usize,
}

fn get_answer(input: &str) -> Option<usize> {
    // get input
    let bots = input
        .lines()
        .map(|line| {
            line.split(['>', '<', ',', '='])
                .filter_map(|x| x.parse::<isize>().ok())
                .collect::<Vec<_>>()
        })
        .map(|x| Bot {
            x: x[0],
            y: x[1],
            z: x[2],
            r: x[3] as usize,
        })
        .collect::<Vec<Bot>>();

    let bigbot = bots.iter().max_by_key(|b| b.r).unwrap();
    let mut power = 0;
    for b in &bots {
        if manhattan_dist(bigbot, b) <= bigbot.r {
            power += 1;
        }
    }
    Some(power)
}

fn manhattan_dist(b1: &Bot, b2: &Bot) -> usize {
    b1.x.abs_diff(b2.x) + b1.y.abs_diff(b2.y) + b1.z.abs_diff(b2.z)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_total() {
        assert_eq!(
            get_answer(include_str!("../assets/day23_input_demo1.txt")),
            Some(7)
        );
        assert_eq!(get_answer(include_str!("../assets/day23_input.txt")), Some(380));
    }
}
