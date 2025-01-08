use std::{
    collections::{HashMap, HashSet, VecDeque},
    time::Instant,
};

// personal functions
//use crate::utils::grid2d;
//use crate::utils::pause;
//use crate::utils::math;

pub fn main() {
    println!("-- Advent of Code - Day 20 - Part 1 --");
    let now = Instant::now();

    // let input = include_str!("../assets/day20_input_demo1.txt");
    // let input = include_str!("../assets/day20_input_demo2.txt");
    // let input = include_str!("../assets/day20_input_demo3.txt");
    // let input = include_str!("../assets/day20_input_demo4.txt");
    // let input = include_str!("../assets/day20_input_demo5.txt");
    let input = include_str!("../assets/day20_input.txt");

    if let Some(answer) = get_answer(input) {
        println!("The answer is : {}", answer);
    } else {
        println!("No answer found");
    }
    let elapsed1 = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed1);
}

// 255 too low

fn get_answer(input: &str) -> Option<usize> {
    let map = input.chars().skip(1).collect::<Vec<char>>();
    // println!("map is {:?}", map);

    let mut rooms: HashMap<(isize, isize), HashSet<(isize, isize)>> = HashMap::new();
    get_all_rooms(&mut rooms, map);

    // draw_rooms(&rooms);

    let mut max_doors = 0;
    for room in &rooms {
        if let Some(doors) = breadth_first_search(&rooms, (0, 0), *room.0) {
            if doors > max_doors {
                max_doors = doors;
            }
        }
    }
    Some(max_doors)
}

fn breadth_first_search(
    rooms: &HashMap<(isize, isize), HashSet<(isize, isize)>>,
    start_node: (isize, isize),
    end_node: (isize, isize),
) -> Option<usize> {
    let mut dejavu = HashSet::new();
    let mut queue = VecDeque::new();

    dejavu.insert(start_node);
    queue.push_back((start_node, 0)); // first cost is 0
    while let Some((current_node, cost)) = queue.pop_front() {
        // trouvé
        if current_node == end_node {
            return Some(cost);
        }

        // Check the neighboring nodes for any that we've not visited yet.
        for room in rooms.get(&current_node).unwrap() {
            if !dejavu.contains(room) {
                dejavu.insert(*room);
                queue.push_back((*room, cost + 1));
            }
        }
    }
    // not found
    None
}

fn get_all_rooms(rooms: &mut HashMap<(isize, isize), HashSet<(isize, isize)>>, map: Vec<char>) {
    let start = (0, 0);
    let mut current = start;
    rooms.insert(current, HashSet::new());
    let mut index = 0;

    let mut queue = VecDeque::new();
    queue.push_back((current, index));

    'out2: while !queue.is_empty() {
        (current, index) = queue.pop_front().unwrap();
        // println!(
        //     "now at map {} ({}) at room {:?}",
        //     index, map[index], current
        // );
        for p in index..map.len() {
            let pos;
            match map[p] {
                'W' => pos = Some((current.0, current.1 - 1)),
                'E' => pos = Some((current.0, current.1 + 1)),
                'N' => pos = Some((current.0 - 1, current.1)),
                'S' => pos = Some((current.0 + 1, current.1)),
                '(' => {
                    // add every possibilities (x...|x...|x)
                    let mut level = 0;
                    // println!("  add queue ({:?},{})", current, p + 1);
                    queue.push_back((current, p + 1));
                    for (p2, _) in map.iter().enumerate().skip(p + 1) {
                        match map[p2] {
                            '|' => {
                                if level == 0 {
                                    // println!("  add queue ({:?},{})", current, p2 + 1);
                                    queue.push_back((current, p2 + 1));
                                }
                            }
                            '(' => level += 1,
                            ')' => {
                                if level == 0 {
                                    continue 'out2;
                                }
                                level -= 1;
                            }
                            _ => {}
                        }
                    }
                    continue 'out2;
                }
                '$' => break,
                '|' => {
                    // skip all branches until ')'
                    let mut level = 0;
                    for (p2, _) in map.iter().enumerate().skip(p + 1) {
                        match map[p2] {
                            '(' => level += 1,
                            ')' => {
                                if level == 0 {
                                    // println!("  add queue ({:?},{})", current, p2 + 1);
                                    queue.push_back((current, p2 + 1));
                                    continue 'out2;
                                }
                                level -= 1;
                            }
                            _ => {}
                        }
                    }
                    continue 'out2;
                }
                ')' => {
                    continue 'out2;
                }
                _ => {
                    panic!()
                }
            };
            if let Some(pos2) = pos {
                // println!("  at {} going {} to room {:?}", p, map[p], pos);
                let next = rooms.entry(pos2).or_default();
                next.insert(current);
                let prev = rooms.entry(current).or_default();
                prev.insert(pos2);
                current = pos2;
            }
        }
    }
}

fn _draw_rooms(rooms: &HashMap<(isize, isize), HashSet<(isize, isize)>>) {
    // get dimensions
    let min_x = rooms
        .iter()
        .min_by(|a, b| a.0 .0.cmp(&b.0 .0))
        .unwrap()
        .0
         .0;
    let max_x = rooms
        .iter()
        .max_by(|a, b| a.0 .0.cmp(&b.0 .0))
        .unwrap()
        .0
         .0;
    let min_y = rooms
        .iter()
        .min_by(|a, b| a.0 .1.cmp(&b.0 .1))
        .unwrap()
        .0
         .1;
    let max_y = rooms
        .iter()
        .max_by(|a, b| a.0 .1.cmp(&b.0 .1))
        .unwrap()
        .0
         .1;
    println!("map is ({},{}) to ({},{})", min_x, min_y, max_x, max_y);

    for x in min_x..=max_x {
        // top line
        for y in min_y..=max_y {
            print!("#");
            if let Some(room) = rooms.get(&(x, y)) {
                if room.contains(&(x - 1, y)) {
                    print!("-");
                } else {
                    print!("#");
                }
            } else {
                print!("#");
            }
        }
        println!("#");

        for y in min_y..=max_y {
            if let Some(current_room) = rooms.get(&(x, y)) {
                if current_room.contains(&(x, y - 1)) {
                    print!("|");
                } else {
                    print!("#");
                }
                if x == 0 && y == 0 {
                    print!("X");
                } else {
                    print!(".");
                }
            } else {
                print!("  ");
            }
        }
        println!("#");

        // bottom line
        if x == max_x {
            for _ in min_y..=max_y {
                print!("##")
            }
            println!("#");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_total() {
        assert_eq!(
            get_answer(include_str!("../assets/day20_input_demo1.txt")),
            Some(3)
        );
        assert_eq!(
            get_answer(include_str!("../assets/day20_input_demo2.txt")),
            Some(10)
        );
        assert_eq!(
            get_answer(include_str!("../assets/day20_input_demo3.txt")),
            Some(18)
        );
        assert_eq!(
            get_answer(include_str!("../assets/day20_input_demo4.txt")),
            Some(23)
        );
        assert_eq!(
            get_answer(include_str!("../assets/day20_input_demo5.txt")),
            Some(31)
        );
        assert_eq!(get_answer(include_str!("../assets/day20_input.txt")), Some(4778));
    }
}
