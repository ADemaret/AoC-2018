use std::time::Instant;

use crate::utils::grid2d::Grid2D;

// personal functions
// use crate::utils::pause;
// use console::Term;

pub fn main() {
    println!("-- Advent of Code - Day 17 - Part 1 --");
    let now = Instant::now();

    // let input = include_str!("../assets/day17_input_demo1.txt");
    // let input = include_str!("../assets/day17_input_test1.txt");
    let input = include_str!("../assets/day17_input.txt");

    if let Some(answer) = get_answer(input) {
        println!("The answers are : {:?}", answer);
    } else {
        println!("No answer found");
    }
    let elapsed1 = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed1);
}

// part1 : 37858
// part2 : 30410

fn get_answer(input: &str) -> Option<(usize,usize)> {
    // get grid size & x offset
    let mut x_min = usize::MAX;
    let mut x_max = 0;
    let mut y_min = usize::MAX;
    let mut y_max = 0;
    input.lines().for_each(|line| {
        let (a, b) = line.split_once(", ").unwrap();
        if a.starts_with("x=") {
            let x = a.strip_prefix("x=").unwrap().parse::<usize>().unwrap();
            x_min = x_min.min(x);
            x_max = x_max.max(x);
        } else {
            let y = a.strip_prefix("y=").unwrap().parse::<usize>().unwrap();
            y_min = y_min.min(y);
            y_max = y_max.max(y);
        }
        let v = b
            .split(['=', '.'])
            .filter_map(|s| s.parse::<usize>().ok())
            .collect::<Vec<_>>();
        let v_min = v[0];
        let v_max = v[1];
        if b.starts_with("x=") {
            x_min = x_min.min(v_min);
            x_max = x_max.max(v_max);
        } else {
            y_min = y_min.min(v_min);
            y_max = y_max.max(v_max);
        }
    });
    // println!(
    //     "grid size is {}<x<{} and {}<y<{}",
    //     x_min, x_max, y_min, y_max
    // );
    let x_offset = x_min;

    let mut grid = Grid2D::new_empty(y_max + 1, x_max - x_min + 2, '.');
    input.lines().for_each(|line| {
        let (a, b) = line.split_once(", ").unwrap();
        if a.starts_with("x=") {
            let x = a.strip_prefix("x=").unwrap().parse::<usize>().unwrap();
            let y = b
                .split(['=', '.'])
                .filter_map(|s| s.parse::<usize>().ok())
                .collect::<Vec<_>>();
            let y_min = y[0];
            let y_max = y[1];
            for y in y_min..=y_max {
                grid.set_at((y, x - x_offset), '#');
            }
        } else {
            let y = a.strip_prefix("y=").unwrap().parse::<usize>().unwrap();
            let x = b
                .split(['=', '.'])
                .filter_map(|s| s.parse::<usize>().ok())
                .collect::<Vec<_>>();
            let x_min = x[0];
            let x_max = x[1];
            for x in x_min..=x_max {
                grid.set_at((y, x - x_offset), '#');
            }
        }
    });

    // source is at x = 500
    grid.set_at((0, 500 - x_offset), '|');
    loop {
        let mut modif = 0;
        let mut water = grid.get_vec_of_char_positions('|');
        water.sort();
        water.reverse();
        for source in water {
            // println!("a source : {:?}", source);
            if source.0 >= grid.max_l - 1 {
                continue;
            }
            match grid.get_at(((source.0) + 1, source.1)) {
                '#' => {
                    modif += water_spread(&mut grid, source);
                }
                '~' => {
                    modif += water_spread(&mut grid, source);
                }
                '.' => {
                    modif += water_drop(&mut grid, source);
                }
                _ => {}
            }
            // grid.print();
            // if source.0 > 54 {
            //     grid.print_sub(50,100,0,500-481+40);
            //     pause::pause();
            // }
        }
        if modif == 0 {
            break;
        }
    }

    
    // to navigate through the grid
    // let mut upleft = (0, 0);
    // let depl = 10;
    // let stdout = Term::buffered_stdout();
    // loop {
    //     grid.print_sub(upleft.0, upleft.0 + 50, upleft.1, upleft.1 + 50);        
    //     if let Ok(character) = stdout.read_char() {
    //         match character {
    //             'l' => {
    //                 if upleft.1 >= depl {
    //                     upleft.1 -= depl;
    //                 }
    //             }
    //             'm' => {
    //                 upleft.1 += depl;
    //             }
    //             'i' => {
    //                 if upleft.0 >= depl {
    //                     upleft.0 -= depl;
    //                 }
    //             }
    //             'k' => {
    //                 upleft.0 += depl;
    //             }
    //             'q' => break,
    //             _ => println!("Invalid input"),
    //         }
    //     }
    // }

    Some((grid.count_occurences('|') + grid.count_occurences('~') - y_min,
    grid.count_occurences('~')))
}

fn water_spread(grid: &mut Grid2D, source: (usize, usize)) -> usize {
    let mut modif = 0;
    // println!("spread at {:?}", source);
    let mut overflow_right = false;
    let mut overflow_left = false;
    let mut fill_right = false;
    let mut fill_left = false;

    let mut x_min = source.0;
    let mut x_max = source.0;

    // if source.0 == 1088 {
    //     grid.print_sub(1085, 1095, 200, 220);
    //     // pause::pause();
    // }

    // left
    for x in (1..=source.1).rev() {
        if grid.get_at((source.0, x - 1)) == '#' {
            fill_left = true;
            // println!("fill left");
            x_min = x;
            break;
        } else if grid.get_at((source.0 + 1, x - 1)) == '.' {
            if grid.get_at((source.0 + 1, x)) == '#' {
                overflow_left = true;
                // println!("overflow left");
                x_min = x - 1;
            } else {
                x_min = x;
            }
            break;
        } else if grid.get_at((source.0 + 1, x - 1)) == '|' {
            overflow_left = true;
            x_min = x;
            break;
        }
    }

    // right
    for x in source.1..=grid.max_c - 2 {
        if grid.get_at((source.0, x + 1)) == '#' {
            fill_right = true;
            // println!("fill right");
            x_max = x;
            break;
        } else if grid.get_at((source.0 + 1, x + 1)) == '.' {
            if grid.get_at((source.0 + 1, x)) == '#' {
                overflow_right = true;
                // println!("overflow right");
                x_max = x + 1;
            } else {
                x_max = x;
            }
            break;
        } else if grid.get_at((source.0 + 1, x + 1)) == '|' {
            overflow_right = true;
            x_max = x;
            break;
        }
    }

    if fill_left && fill_right {
        for x in x_min..=x_max {
            if grid.get_at((source.0, x)) != '~' {
                grid.set_at((source.0, x), '~');
                modif += 1;
            }
        }
    } else if overflow_left || overflow_right {
        for x in x_min..=x_max {
            if x < grid.max_c && grid.get_at((source.0, x)) != '|' {
                grid.set_at((source.0, x), '|');
                modif += 1;
            }
        }
    }
    modif
}

fn water_drop(grid: &mut Grid2D, source: (usize, usize)) -> usize {
    let mut modif = 0;
    // println!("drop at {:?}", source);
    for y in source.0 + 1..grid.max_l {
        if grid.get_at((y, source.1)) == '.' {
            grid.set_at((y, source.1), '|');
            modif += 1;
        } else {
            break;
        }
    }
    modif
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_total() {
        assert_eq!(
            get_answer(include_str!("../assets/day17_input_demo1.txt")),
            Some((57,29))
        );
        assert_eq!(
            get_answer(include_str!("../assets/day17_input.txt")),
            Some((37858,30410))
        );
    }
}
