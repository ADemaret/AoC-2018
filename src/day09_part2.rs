use std::time::Instant;

// personal functions
//use crate::utils::grid2d;
//use crate::utils::pause;
//use crate::utils::math;

pub fn main() {
    println!("-- Advent of Code - Day 09 - Part 2 --");
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
    let max_marble = chunks[6].parse::<usize>().unwrap() * 100;

    let mut game_chunks: Vec<(usize, Vec<usize>)> = Vec::new();
    game_chunks.push((0, vec![0]));
    let mut game_len = 1;

    let mut player = 0;
    let mut scores = vec![0; nbr_players];
    let mut current = 0;
    // println!("[-] (0)");

    for i in 1..=max_marble {
        if i % 100000 == 0 {
            println!("{i} / {max_marble}");
        }
        player = (player + 1) % nbr_players;
        if i % 23 == 0 {
            scores[player] += i;
            current = (current + game_len - 7) % game_len;
            scores[player] += game_get_and_remove(&mut game_chunks, current);
            // current = current % game.len() +1;
            game_len -= 1;
        } else {
            current = (current + 1) % game_len + 1;

            game_insert(&mut game_chunks, current, i);
            game_len += 1;
        }

        // print!("[{}]", player + 1);
        // for gc in &game_chunks {
        //     for (x, g) in gc.1.iter().enumerate() {
        //         if gc.0 + x == current {
        //             print!(" ({})", g);
        //         } else {
        //             print!(" {:2}", g);
        //         }
        //     }
        // }
        // println!(" ");
    }
    // println!("Scores : {:?}",scores);
    Some(*scores.iter().max().unwrap())
}

fn game_get_and_remove(game_chunks: &mut [(usize, Vec<usize>)], pos: usize) -> usize {
    for gci in 0..game_chunks.len() {
        if pos >= game_chunks[gci].0
            && (gci == game_chunks.len() - 1 || pos < game_chunks[gci + 1].0)
        {
            let start_index = game_chunks[gci].0;
            let result = game_chunks[gci].1[pos - start_index];
            game_chunks[gci].1.remove(pos - start_index);
            if gci < game_chunks.len() - 1 {
                for gci2 in gci + 1..game_chunks.len() {
                    game_chunks[gci2].0 -= 1;
                }
            }
            return result;
        }
    }
    0
}

fn game_insert(game_chunks: &mut Vec<(usize, Vec<usize>)>, pos: usize, marble: usize) {
    let max_len = 10000;
    for gci in 0..game_chunks.len() {
        if pos >= game_chunks[gci].0
            && (gci == game_chunks.len() - 1 || pos < game_chunks[gci + 1].0)
        {
            let start_index = game_chunks[gci].0;

            game_chunks[gci].1.insert(pos - start_index, marble);

            if game_chunks[gci].1.len() > max_len {
                let prev_offset = game_chunks[gci].0;
                let v2 = game_chunks[gci].1.split_off(max_len / 2);
                game_chunks.insert(gci + 1, ((prev_offset + (max_len / 2) - 1), v2));
            }
            if gci < game_chunks.len() - 1 {
                for gci2 in gci + 1..game_chunks.len() {
                    game_chunks[gci2].0 += 1;
                }
            }
            return;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part2() {
        // to test with max-len = 4 at line 95
        // let mut game_chunks = vec![(0, vec![0, 1, 2]), (3, vec![3, 4, 5]), (6, vec![6, 7, 8])];
        // assert_eq!(game_get_and_remove(&mut game_chunks, 1), 1);
        // assert_eq!(game_get_and_remove(&mut game_chunks, 3), 4);
        // assert_eq!(game_get_and_remove(&mut game_chunks, 6), 8);

        // let mut game_chunks = vec![(0, vec![0, 1, 2]), (3, vec![3, 4, 5]), (6, vec![6, 7, 8])];
        // game_insert(&mut game_chunks, 3, 0);
        // assert_eq!(
        //     game_chunks,
        //     vec![
        //         (0, vec![0, 1, 2]),
        //         (3, vec![0, 3, 4, 5]),
        //         (7, vec![6, 7, 8])
        //     ]
        // );
        // game_insert(&mut game_chunks, 3, 0);
        // assert_eq!(
        //     game_chunks,
        //     vec![
        //         (0, vec![0, 1, 2]),
        //         (3, vec![0, 0]),
        //         (5, vec![3, 4, 5]),
        //         (8, vec![6, 7, 8])
        //     ]
        // );
    }

    #[test]
    fn test_total() {
        assert_eq!(
            get_answer(include_str!("../assets/day09_input.txt")),
            Some(3505711612)
        );
    }
}
