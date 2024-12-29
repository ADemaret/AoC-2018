use std::time::Instant;

use crate::utils::grid2d::Grid2D;

// personal functions
//use crate::utils::grid2d;
// use crate::utils::pause;
//use crate::utils::math;

pub fn main() {
    println!("-- Advent of Code - Day 10 - Part 1 --");
    let now = Instant::now();

    let input = include_str!("../assets/day10_input_demo1.txt");
    // let input = include_str!("../assets/day10_input.txt");

    if let Some(answer) = get_answer(input) {
        println!("The answer is : {}", answer);
    } else {
        println!("No answer found");
    }
    let elapsed1 = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed1);
}

//

#[derive(Default, Debug, Copy, Clone)]
struct Point {
    pos: (i32, i32),
    vel: (i32, i32),
}

fn get_answer(input: &str) -> Option<usize> {
    let mut points = input
        .lines()
        .map(|line| {
            let chunks = line
                .split(['<', ',', ' ', '>'])
                .filter(|x| !x.is_empty())
                .collect::<Vec<_>>();
            Point {
                pos: (
                    chunks[1].parse::<i32>().unwrap(),
                    chunks[2].parse::<i32>().unwrap(),
                ),
                vel: (
                    chunks[4].parse::<i32>().unwrap(),
                    chunks[5].parse::<i32>().unwrap(),
                ),
            }
        })
        .collect::<Vec<_>>();

    // println!("{:?}", points);

    let best_index = get_best_index(&mut points.clone());
    println!("Best index is {}", best_index);

    let mut p0_min = i32::MAX;
    let mut p0_max = 0;
    let mut p1_min = i32::MAX;
    let mut p1_max = 0;
    for p in points.iter_mut() {
        p.pos.0 += p.vel.0 * best_index;
        p.pos.1 += p.vel.1 * best_index;
        p0_min = p0_min.min(p.pos.0);
        p0_max = p0_max.max(p.pos.0);
        p1_min = p1_min.min(p.pos.1);
        p1_max = p1_max.max(p.pos.1);
    }
    // println!("p0 is between {} and {}", p0_min, p0_max);
    // println!("p1 is between {} and {}", p1_min, p1_max);

    let grid = Grid2D::new_empty((p1_max-p1_min+1) as usize, (p0_max-p0_min+1) as usize, '.');

    let v = points
        .iter()
        // .filter(|p| p.pos.0 >= p0_min && p.pos.1 >= p1_min)
        .map(|p| ((p.pos.1 - p1_min) as usize, (p.pos.0 - p0_min) as usize))
        .collect::<Vec<(usize, usize)>>();
    // println!("points = {:?}", v);
    grid.print_with_vec(&v, 'X');
    None
}

fn get_best_index(points: &mut [Point]) -> i32 {
    let mut best_w = i32::MAX;
    // let mut best_h = i32::MAX;
    let mut min_width = i32::MAX;
    // let mut min_height = i32::MAX;
    for i in 1..12200 {
        let mut p0_min = 0;
        let mut p0_max = 0;
        let mut p1_min = 0;
        let mut p1_max = 0;
        for p in points.iter_mut() {
            p.pos.0 += p.vel.0;
            p.pos.1 += p.vel.1;
            p0_min = p0_min.min(p.pos.0);
            p0_max = p0_max.max(p.pos.0);
            p1_min = p1_min.min(p.pos.1);
            p1_max = p1_max.max(p.pos.1);
        }
        if p0_max - p0_min < min_width {
            min_width = p0_max - p0_min;
            best_w = i;
        }
        // if p1_max - p1_min < min_height {
        //     min_height = p1_max - p1_min;
        //     best_h = i;
        // }
        // println!("best width is {} at {}", min_width, best_w);
        // println!("best height is {} at {}", min_height, best_h);

        //println!("at {}, coords are in ({},{}), ({},{})", i*100,p0_min,p1_min,p0_max,p1_max);
    }
    best_w
}
