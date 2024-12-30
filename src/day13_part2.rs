use std::time::Instant;

// personal functions
use crate::utils::grid2d::Grid2D;
// use crate::utils::pause;
//use crate::utils::math;

pub fn main() {
    println!("-- Advent of Code - Day 13 - Part 2 --");
    let now = Instant::now();

    // let input = include_str!("../assets/day13_input_demo2.txt");
    let input = include_str!("../assets/day13_input.txt");

    if let Some(answer) = get_answer(input) {
        println!("The answer is : {:?}", answer);
    } else {
        println!("No answer found");
    }
    let elapsed1 = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed1);
}

//

#[derive(Debug)]
enum Turn {
    Left,
    Straight,
    Right,
}

#[derive(Debug)]
struct Bot {
    pos: (usize, usize),
    dir: (isize, isize),
    next_turn: Turn,
    id: usize,
    deleted: bool,
}

fn get_answer(input: &str) -> Option<(usize, usize)> {
    let mut grid = Grid2D::new(input);
    let mut bots: Vec<Bot> = Vec::new();

    get_bots(&mut grid, &mut bots, '>', (0, 1), '-');
    get_bots(&mut grid, &mut bots, '<', (0, -1), '-');
    get_bots(&mut grid, &mut bots, '^', (-1, 0), '|');
    get_bots(&mut grid, &mut bots, 'v', (1, 0), '|');

    // let v = bots.iter().map(|b| b.pos).collect::<Vec<(usize, usize)>>();
    // grid.print_with_vec(&v, 'O');
    // pause::pause();

    let mut l: u64 = 0;
    loop {
        l += 1;
        bots.sort_by_key(|b| (b.pos.0, b.pos.1));

        for i in 0..bots.len() {
            if !bots[i].deleted {
                move_bot(&grid, &mut bots[i]);
                delete_crashed(i, &mut bots);
            }
        }

        // let v = bots.iter().filter(|&x| !x.deleted).map(|b| b.pos).collect::<Vec<(usize, usize)>>();
        // grid.print_with_vec(&v, 'O');
        // pause::pause();

        let remaining = bots.iter().filter(|&x| !x.deleted).collect::<Vec<_>>();
        if remaining.len() == 1 {
            println!("{l}");
            return Some((remaining[0].pos.1, remaining[0].pos.0));
        }
    }
}

fn delete_crashed(index: usize, bots: &mut [Bot]) {
    for i in 0..bots.len() {
        if !bots[i].deleted
            && bots[index].id != bots[i].id
            && bots[index].pos.0 == bots[i].pos.0
            && bots[index].pos.1 == bots[i].pos.1
        {
            bots[index].deleted = true;
            bots[i].deleted = true;
        }
    }
}

fn move_bot(grid: &Grid2D, bot: &mut Bot) {
    let coord_next = (
        (bot.pos.0 as isize + bot.dir.0) as usize,
        (bot.pos.1 as isize + bot.dir.1) as usize,
    );
    bot.pos = coord_next;
    let cell_next = grid.get_at(coord_next);
    match (bot.dir, cell_next) {
        // >
        ((0, 1), '-') => {}
        ((0, 1), '\\') => {
            bot.dir = (1, 0);
        }
        ((0, 1), '/') => {
            bot.dir = (-1, 0);
        }
        ((0, 1), '+') => match bot.next_turn {
            Turn::Left => {
                bot.dir = (-1, 0);
                bot.next_turn = Turn::Straight;
            }
            Turn::Straight => {
                bot.next_turn = Turn::Right;
            }
            Turn::Right => {
                bot.dir = (1, 0);
                bot.next_turn = Turn::Left;
            }
        },
        // v
        ((1, 0), '|') => {}
        ((1, 0), '\\') => {
            bot.dir = (0, 1);
        }
        ((1, 0), '/') => {
            bot.dir = (0, -1);
        }
        ((1, 0), '+') => match bot.next_turn {
            Turn::Left => {
                bot.dir = (0, 1);
                bot.next_turn = Turn::Straight;
            }
            Turn::Straight => {
                bot.next_turn = Turn::Right;
            }
            Turn::Right => {
                bot.dir = (0, -1);
                bot.next_turn = Turn::Left;
            }
        },
        // <
        ((0, -1), '-') => {}
        ((0, -1), '\\') => {
            bot.dir = (-1, 0);
        }
        ((0, -1), '/') => {
            bot.dir = (1, 0);
        }
        ((0, -1), '+') => match bot.next_turn {
            Turn::Left => {
                bot.dir = (1, 0);
                bot.next_turn = Turn::Straight;
            }
            Turn::Straight => {
                bot.next_turn = Turn::Right;
            }
            Turn::Right => {
                bot.dir = (-1, 0);
                bot.next_turn = Turn::Left;
            }
        },
        // ^
        ((-1, 0), '|') => {}
        ((-1, 0), '\\') => {
            bot.dir = (0, -1);
        }
        ((-1, 0), '/') => {
            bot.dir = (0, 1);
        }
        ((-1, 0), '+') => match bot.next_turn {
            Turn::Left => {
                bot.dir = (0, -1);
                bot.next_turn = Turn::Straight;
            }
            Turn::Straight => {
                bot.next_turn = Turn::Right;
            }
            Turn::Right => {
                bot.dir = (0, 1);
                bot.next_turn = Turn::Left;
            }
        },
        _ => panic!(),
    }
}

fn get_bots(
    grid: &mut Grid2D,
    bots: &mut Vec<Bot>,
    symb: char,
    dir2: (isize, isize),
    new_symb: char,
) {
    let v1 = grid.get_vec_of_char_positions(symb);
    let mut nbr = bots.len();
    for v in v1 {
        bots.push(Bot {
            pos: (v.0, v.1),
            dir: dir2,
            next_turn: Turn::Left,
            id: nbr,
            deleted: false,
        });
        nbr += 1;
        grid.set_at((v.0, v.1), new_symb);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_total() {
        assert_eq!(
            get_answer(include_str!("../assets/day13_input_demo2.txt")),
            Some((6, 4))
        );
        assert_eq!(
            get_answer(include_str!("../assets/day13_input.txt")),
            Some((113, 109))
        );
    }
}
