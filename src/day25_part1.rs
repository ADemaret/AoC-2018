use std::time::Instant;

// personal functions
//use crate::utils::grid2d;
//use crate::utils::pause;
//use crate::utils::math;

pub fn main() {
    println!("-- Advent of Code - Day 25 - Part 1 --");
    let now = Instant::now();

    // let input = include_str!("../assets/day25_input_demo1.txt");
    let input = include_str!("../assets/day25_input.txt");

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
    let mut points = input
        .lines()
        .map(|line| {
            let coords = line
                .split(",")
                .filter_map(|x| x.parse::<isize>().ok())
                .collect::<Vec<_>>();
            (0, coords[0], coords[1], coords[2], coords[3])
        })
        .collect::<Vec<(usize, isize, isize, isize, isize)>>();

    // for p in &points {
    //     println!("{:?}", p);
    // }

    let mut constellation = 1;
    points[0].0 = constellation;
    loop {
        loop {
            let mut added = 0;
            for idx1 in 0..points.len() {
                if points[idx1].0 == constellation {
                    for idx2 in 0..points.len() {
                        if idx1 != idx2
                            && points[idx2].0 == 0
                            && manhattan_dist(points[idx1], points[idx2]) <= 3
                        {
                            points[idx2].0 = constellation;
                            added += 1;
                        }
                    }
                }
            }
            if added == 0 {
                break;
            }
        }

        constellation += 1;
        let mut new_constellation = false;
        for pt in points.iter_mut() {
            if !new_constellation && pt.0 == 0 {
                pt.0 = constellation;
                new_constellation = true;
            }
        }
        if !new_constellation {
            // for (i, pt) in points.iter().enumerate() {
            //     println!("pt {i} is in constellation {}", pt.0);
            // }
            return Some(constellation-1);
        }
    }
}

fn manhattan_dist(
    pt1: (usize, isize, isize, isize, isize),
    pt2: (usize, isize, isize, isize, isize),
) -> usize {
    pt1.1.abs_diff(pt2.1) + pt1.2.abs_diff(pt2.2) + pt1.3.abs_diff(pt2.3) + pt1.4.abs_diff(pt2.4)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_total() {
        assert_eq!(
            get_answer(include_str!("../assets/day25_input_demo1.txt")),
            Some(8)
        );
        assert_eq!(get_answer(include_str!("../assets/day25_input.txt")), Some(324));
    }
}
