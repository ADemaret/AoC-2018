use std::time::Instant;

// personal functions
//use crate::utils::grid2d;
//use crate::utils::pause;
//use crate::utils::math;

pub fn main() {
    println!("-- Advent of Code - Day 02 - Part 2 --");
    let now = Instant::now();

    //let input = include_str!("../assets/day02_input_demo2.txt");
    let input = include_str!("../assets/day02_input.txt");

    println!("La réponse est {:?}", get_answer(input));
    //get_answer(input);

    let elapsed1 = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed1);
}

//

fn get_answer(input: &str) -> String {
    let mut prev_lines: Vec<Vec<char>> = Vec::new();
    let mut str = String::new();
    
    for line in input.lines() {
        let this_line: Vec<char> = line.chars().collect();
        prev_lines.push(this_line.clone());

        for pl in &prev_lines {
            let mut diffs = 0;
            for index in 0..line.len() {
                if pl[index] != this_line[index] {
                    diffs+=1;
                }
            }
            if diffs==1 {
                // println!("{:?}",pl);
                // println!("{:?}",this_line);
                for index in 0..line.len() {
                    if pl[index] == this_line[index] {
                        // print!("{}",pl[index]);
                        str.push(pl[index]);
                    }
                }
                //println!();
                //println!("found !!");
                return str;
            }
        }
    }
    str
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_total() {
        assert_eq!(
            get_answer(include_str!("../assets/day02_input_demo2.txt")),
            "fgij"
        );
        assert_eq!(get_answer(include_str!("../assets/day02_input.txt")), "bvnfawcnyoeyudzrpgslimtkj");
    }
}
