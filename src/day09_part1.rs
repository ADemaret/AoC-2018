use std::time::Instant;

// personal functions
//use crate::utils::grid2d;
//use crate::utils::pause;
//use crate::utils::math;

pub fn main() {
    println!("-- Advent of Code - Day 09 - Part 1 --");
    let now = Instant::now();

    // let input = "9 players; last marble is worth 25 points";
    // let input = "10 players; last marble is worth 1618 points";
    let input = include_str!("../assets/day09_input.txt");

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
    let chunks: Vec<&str> = input.split_whitespace().collect();
    let nbr_players = chunks[0].parse::<usize>().unwrap();
    let max_marble = chunks[6].parse::<usize>().unwrap();

    let mut game = vec![0];
    let mut player = 0;
    let mut scores = vec![0; nbr_players];
    let mut current = 0;
    // println!("[-] (0)");

    for i in 1..=max_marble {
        player = (player +1) % nbr_players;
        if i % 23 == 0 {
            scores[player] += i;
            current = (current + game.len() - 7) % game.len();
            scores[player] += game[current];
            game.remove(current);
            // current = current % game.len() +1;
        } else {
            current = (current + 1) % game.len() + 1;
            game.insert(current, i);
        }

        // print!("[{}]",player+1);
        // for (x, g) in game.iter().enumerate() {
        //     if x == current {
        //         print!(" ({})",g);
        //     } else {
        //         print!(" {:2}",g);
        //     }
        // }
        // println!(" ");
    }
    // println!("Scores : {:?}",scores);
    Some(*scores.iter().max().unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_total() {
        assert_eq!(
            get_answer("9 players; last marble is worth 25 points"),
            Some(32)
        );

        assert_eq!(
            get_answer("10 players; last marble is worth 1618 points"),
            Some(8317)
        );
        assert_eq!(
            get_answer("13 players; last marble is worth 7999 points"),
            Some(146373)
        );
        assert_eq!(
            get_answer("17 players; last marble is worth 1104 points"),
            Some(2764)
        );
        assert_eq!(
            get_answer("21 players; last marble is worth 6111 points"),
            Some(54718)
        );
        assert_eq!(
            get_answer("30 players; last marble is worth 5807 points"),
            Some(37305)
        );

        // assert_eq!(get_answer(include_str!("../assets/day09_input.txt")), None);
    }
}
