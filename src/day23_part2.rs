use std::time::Instant;

// personal functions
//use crate::utils::grid2d;
//use crate::utils::pause;
//use crate::utils::math;

pub fn main() {
    println!("-- Advent of Code - Day 23 - Part 2 --");
    let now = Instant::now();

    // let input = include_str!("../assets/day23_input_demo2.txt");
    let input = include_str!("../assets/day23_input.txt");

    if let Some(answer) = get_answer(input,1_000_000) {
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
    r: isize,
}

fn get_answer(input: &str,zoom:isize) -> Option<isize> {
    let mut zoom_factor = zoom;
    let org_zoom_factor = zoom_factor;
    let mut sol = (0, 0, 0);

    let mut x_mid = 0;
    let mut y_mid = 0;
    let mut z_mid = 0;

    while zoom_factor >= 1 {
        // get input
        let bots = input
            .lines()
            .map(|line| {
                line.split(['>', '<', ',', '='])
                    .filter_map(|x| x.parse::<isize>().ok())
                    .collect::<Vec<_>>()
            })
            .map(|x| Bot {
                x: x[0] / zoom_factor,
                y: x[1] / zoom_factor,
                z: x[2] / zoom_factor,
                r: x[3] / zoom_factor,
            })
            .collect::<Vec<Bot>>();

        let x_min = bots.iter().min_by_key(|b| b.x).unwrap().x;
        let x_max = bots.iter().max_by_key(|b| b.x).unwrap().x;
        let y_min = bots.iter().min_by_key(|b| b.y).unwrap().y;
        let y_max = bots.iter().max_by_key(|b| b.y).unwrap().y;
        let z_min = bots.iter().min_by_key(|b| b.z).unwrap().z;
        let z_max = bots.iter().max_by_key(|b| b.z).unwrap().z;

        if zoom_factor == org_zoom_factor {
            x_mid = (x_min + x_max) / 2;
            y_mid = (y_min + y_max) / 2;
            z_mid = (z_min + z_max) / 2;
            println!(
                "at zoom factor {zoom_factor}, zone is {},{} {},{} {},{}",
                x_min, x_max, y_min, y_max, z_min, z_max
            );
        } else {
            x_mid *= 10;
            y_mid *= 10;
            z_mid *= 10;
            println!(
                "at zoom factor {zoom_factor}, zone is around {},{},{}",
                x_mid, y_mid, z_mid
            );
        }

        let x_min = x_mid - 100;
        let x_max = x_mid + 100;
        let y_min = y_mid - 100;
        let y_max = y_mid + 100;
        let z_min = z_mid - 100;
        let z_max = z_mid + 100;

        let mut best = Vec::new();
        let mut nbr_max = 0;
        for x in x_min..=x_max {
            for y in y_min..=y_max {
                for z in z_min..=z_max {
                    let me = Bot { x, y, z, r: 0 };
                    let mut nbr = 0;
                    for b in &bots {
                        if manhattan_dist(b, &me) <= b.r {
                            nbr += 1;
                        }
                    }
                    if nbr.cmp(&nbr_max).is_gt() {
                        nbr_max = nbr;
                        best = Vec::new();
                        best.push((x, y, z));
                    } else if nbr == nbr_max {
                        best.push((x, y, z));
                    }
                }
            }
        }

        sol = *best.iter().min_by_key(|b| b.0 + b.1 + b.2).unwrap();
        print!("I can be in range of {nbr_max} bots at ");
        println!("{:?}", sol);
        if sol.0 == x_mid && sol.1 == y_mid && sol.2 == z_mid {
            break;
        }
        x_mid = sol.0;
        y_mid = sol.1;
        z_mid = sol.2;

        zoom_factor /= 10;
    }
    Some(sol.0.abs() + sol.1.abs() + sol.2.abs())
}

fn manhattan_dist(b1: &Bot, b2: &Bot) -> isize {
    (b1.x.abs_diff(b2.x) + b1.y.abs_diff(b2.y) + b1.z.abs_diff(b2.z)) as isize
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_total() {
        assert_eq!(
            get_answer(include_str!("../assets/day23_input_demo2.txt"),1),
            Some(36)
        );
        assert_eq!(
            get_answer(include_str!("../assets/day23_input.txt"),1_000_000),
            Some(95541011)
        );
    }
}
