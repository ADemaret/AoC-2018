use crate::utils::grid2d::Grid2D;
use std::{collections::VecDeque, time::Instant};

// personal functions
//use crate::utils::grid2d;
use crate::utils::pause;
//use crate::utils::math;

pub fn main() {
    println!("-- Advent of Code - Day 15 - Part 1 --");
    let now = Instant::now();

    // let input = include_str!("../assets/day15_input_demo1.txt");
    // let input = include_str!("../assets/day15_input_demo2.txt");
    // let input = include_str!("../assets/day15_input_demo3.txt");
    // let input = include_str!("../assets/day15_input_demo4.txt");
    // let input = include_str!("../assets/day15_input_demo5.txt");
    let input = include_str!("../assets/day15_input_demo6.txt");
    // let input = include_str!("../assets/day15_input.txt");

    if let Some(answer) = get_answer(input) {
        println!("The answer is : {}", answer);
    } else {
        println!("No answer found");
    }
    let elapsed1 = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed1);
}

// 

#[derive(Debug, Default, Clone, Copy)]
struct Unit {
    id: usize,
    pos: (usize, usize),
    side: char,
    alive: bool,
    attack: isize,
    hit: isize,
}

fn get_answer(input: &str) -> Option<usize> {
    // get grid
    let mut grid = Grid2D::new(input);

    // get units
    let mut units = get_units(&grid);

    let mut round = 0;
    loop {
        // sorted_units is the sorted id of the units (sorted by position on the grid)
        let mut sorted_units = Vec::new();
        for u in &units {
            sorted_units.push((u.pos, u.id));
        }
        sorted_units.sort_by_key(|u| (u.0 .0, u.0 .1));
        print_situation(&grid, round, &units, &sorted_units);

        round += 1;
        println!("round {} begins", round);

        for su in &sorted_units {
            let current = su.1;

            if !units[current].alive {
                continue;
            }

            // print active unit
            for x in &sorted_units {
                if units[x.1].alive && x.1 == current {
                    println!(
                        "-> {} at {:?} : hp = {}",
                        units[x.1].side, units[x.1].pos, units[x.1].hit
                    );
                }
            }

            let opponent = match units[current].side {
                'E' => 'G',
                'G' => 'E',
                _ => panic!(),
            };
            let range = &grid.get_adjacents_ortho(units[current].pos.0, units[current].pos.1);
            let v_free = range.iter().filter(|&r| r.2 == '.').collect::<Vec<_>>();
            let v_opp = range
                .iter()
                .filter(|&r| r.2 == opponent)
                .collect::<Vec<_>>();
            // if no opponent is in range and it is possible to move
            if v_opp.is_empty() && !v_free.is_empty() {
                // move
                if let Some(new_pos) = move_one_step(&grid, &units, units[current]) {
                    println!("{:?} moves to {:?}", units[current].pos, new_pos);
                    grid.set_at(units[current].pos, '.');
                    grid.set_at(new_pos, units[current].side);
                    units[current].pos = new_pos;
                } else {
                    println!("{:?} stays", units[current].pos);
                }
            }
            // after move, check if opponent is now in range
            let range = &grid.get_adjacents_ortho(units[current].pos.0, units[current].pos.1);
            let v_opp = range
                .iter()
                .filter(|&r| r.2 == opponent)
                .collect::<Vec<_>>();
            if !v_opp.is_empty() {
                // attack
                if let Some(new_pos) = attack(&grid, &units, units[current]) {
                    let opp_id = get_unit_id_by_pos(&units, new_pos);
                    units[opp_id].hit -= units[current].attack;
                    print!(
                        "{:?} attack opponent at pos {:?}",
                        units[current].pos, new_pos
                    );
                    if units[opp_id].hit <= 0 {
                        println!("opp is dead");
                        units[opp_id].alive = false;
                        grid.set_at(units[opp_id].pos, '.');
                        if let Some(result) = end_of_game(&grid, &units, &sorted_units, round) {
                            if units[current].id == sorted_units.last().unwrap().1 {
                                println!("all units have been processed - round is finished")
                            } else {
                                println!("round is stopped before all units have been processed")
                            }
                            return Some(result);
                        }
                    } else {
                        println!("opp has now {} hp", units[opp_id].hit);
                    }
                }
            }
            println!("--");
        }
    }
}

fn end_of_game(grid:&Grid2D,units: &[Unit], sorted_units:&[((usize,usize),usize)],round: usize) -> Option<usize> {
    let v_g = units
        .iter()
        .filter(|&u| u.side == 'G' && u.alive)
        .copied()
        .collect::<Vec<_>>();
    let v_e = units
        .iter()
        .filter(|&u| u.side == 'E' && u.alive)
        .copied()
        .collect::<Vec<_>>();
    // println!("v_g : {:?}",v_g);
    // println!("v_e : {:?}",v_e);
    let ghp = v_g.iter().map(|u| u.hit).sum::<isize>();
    let ehp = v_e.iter().map(|u| u.hit).sum::<isize>();

    if v_e.is_empty() {
        println!("------------------------------------");
        print_situation(grid,round, units, sorted_units,);
        println!("------------------------------------");
        println!("at round {}, all elves are dead.", round);
        println!("G hit points = {}", ghp);
        return Some((round) * ghp as usize);
    } else if v_g.is_empty() {
        println!("------------------------------------");
        print_situation(grid, round, units, sorted_units);
        println!("------------------------------------");
        println!("at round {}, all goblins are dead.", round);
        println!("E hit points = {}", ehp);
        return Some((round) * ehp as usize);
    }
    None
}

fn print_situation(grid:&Grid2D, round: usize, units: &[Unit], sorted_units:&[((usize,usize),usize)]) {
    println!("after {} round", round);
    grid.print();
    for x in sorted_units {
        if !units[x.1].alive {
            continue;
        }
        println!(
            "-> {} at {:?} : hp = {}",
            units[x.1].side, units[x.1].pos, units[x.1].hit
        );
    }
    pause::pause();
}

fn get_unit_id_by_pos(units: &Vec<Unit>, pos: (usize, usize)) -> usize {
    for u in units {
        if u.pos == pos && u.alive {
            return u.id;
        }
    }
    panic!();
}

fn attack(grid: &Grid2D, units: &[Unit], u: Unit) -> Option<(usize, usize)> {
    let opponent = match u.side {
        'G' => 'E',
        'E' => 'G',
        _ => panic!(),
    };
    let dir = [(-1, 0), (0, -1), (0, 1), (1, 0)];
    let mut hp = [isize::MAX, isize::MAX, isize::MAX, isize::MAX];
    for (i, d) in dir.iter().enumerate() {
        if let Some(start_pos) = grid.is_next_valid(u.pos, *d) {
            if grid.get_at(start_pos) == opponent {
                // get infos on opponent in range
                for opp in units.iter() {
                    if opp.pos.0 == start_pos.0 && opp.pos.1 == start_pos.1 {
                        hp[i] = opp.hit;
                    }
                }
            }
        }
    }
    // println!("hit points : {:?}", hp);

    let min = hp.iter().min().unwrap();
    let mvt = hp.iter().position(|&x| x == *min).unwrap();
    if let Some(new_pos) = grid.is_next_valid(u.pos, dir[mvt]) {
        return Some(new_pos);
    }
    None
}

fn move_one_step(grid: &Grid2D, units:&Vec<Unit>, u: Unit) -> Option<(usize, usize)> {
    let opponent = match u.side {
        'G' => 'E',
        'E' => 'G',
        _ => panic!(),
    };
    let dir = [(-1, 0), (0, -1), (0, 1), (1, 0)];
    let mut cost = [usize::MAX, usize::MAX, usize::MAX, usize::MAX];
    for (i, d) in dir.iter().enumerate() {
        if let Some(start_pos) = grid.is_next_valid(u.pos, *d) {
            if grid.get_at(start_pos) == '.' {
                if let Some(c) = bfs(grid, units, start_pos, opponent) {
                    cost[i] = c;
                }
            }
        }
    }
    println!("costs : {:?}", cost);
    // pause::pause();

    let min = cost.iter().min().unwrap();
    if *min < usize::MAX {
        let mvt = cost.iter().position(|&x| x == *min).unwrap();
        if let Some(new_pos) = grid.is_next_valid(u.pos, dir[mvt]) {
            return Some(new_pos);
        }
    }
    None
}

fn bfs(grid: &Grid2D, units:&Vec<Unit>, start_pos: (usize, usize), opponent: char) -> Option<usize> {
    let mut dejavu = vec![false; grid.max_l * grid.max_c];
    let mut queue = VecDeque::new();

    let mut vec_of_adj_opp = Vec::new();
    for u in units {
        if u.alive && u.side == opponent {
            for c in grid.get_adjacents_ortho(u.pos.0, u.pos.1) {
                if c.2 == '.' {
                    vec_of_adj_opp.push(c);
                }
            }
        }
    }
    println!("opp range : {:?}",vec_of_adj_opp);

    let current_pos = start_pos;
    dejavu[current_pos.0 * grid.max_l + current_pos.1] = true;
    queue.push_back((current_pos, 0)); // first cost is 0
    while let Some((current_pos, cost)) = queue.pop_front() {
        // trouvé        
        // if grid.get_at(current_pos) == opponent {
        if vec_of_adj_opp.contains(&(current_pos.0,current_pos.1,'.')) {
            // println!("{:?} moves near {:?}", start_pos, current_pos);
            return Some(cost + 1);
        }
        // Check the neighboring nodes for any that we've not visited yet.
        for link in &grid.get_adjacents_ortho(current_pos.0, current_pos.1) {
            if (link.2 == '.' || link.2 == opponent) && !dejavu[link.0 * grid.max_l + link.1] {
                dejavu[link.0 * grid.max_l + link.1] = true;
                queue.push_back(((link.0, link.1), cost + 1));
            }
        }
    }
    // not found
    None
}

fn get_units(grid: &Grid2D) -> Vec<Unit> {
    let mut units: Vec<Unit> = Vec::new();
    add_unit(grid, &mut units, 'G');
    add_unit(grid, &mut units, 'E');
    units
}

fn add_unit(grid: &Grid2D, unit: &mut Vec<Unit>, side: char) {
    for v in grid.get_vec_of_char_positions(side) {
        unit.push(Unit {
            id: unit.len(),
            pos: (v.0, v.1),
            side,
            alive: true,
            attack: 3,
            hit: 200,
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_total() {
        assert_eq!(
            get_answer(include_str!("../assets/day15_input_demo1.txt")),
            Some(27730)
        );
        assert_eq!(
            get_answer(include_str!("../assets/day15_input_demo2.txt")),
            Some(36334)
        );
        assert_eq!(
            get_answer(include_str!("../assets/day15_input_demo3.txt")),
            Some(39514)
        );
        assert_eq!(
            get_answer(include_str!("../assets/day15_input_demo4.txt")),
            Some(27755)
        );
        assert_eq!(
            get_answer(include_str!("../assets/day15_input_demo5.txt")),
            Some(28944)
        );
        assert_eq!(
            get_answer(include_str!("../assets/day15_input_demo6.txt")),
            Some(18740)
        );
        assert_eq!(get_answer(include_str!("../assets/day15_input.txt")), Some(189000));
    }
}
