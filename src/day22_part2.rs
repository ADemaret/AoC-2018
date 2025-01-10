use std::{
    collections::{BTreeMap, HashMap},
    time::Instant,
};

use crate::utils::grid2d::Grid2D;

// personal functions
//use crate::utils::grid2d;
//use crate::utils::pause;
//use crate::utils::math;

pub fn main() {
    println!("-- Advent of Code - Day 22 - Part 2 --");
    let now = Instant::now();

    // let input = include_str!("../assets/day22_input_demo1.txt");
    let input = include_str!("../assets/day22_input.txt");

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
    let (l1, l2) = input.split_once("\n").unwrap();
    let (_, d) = l1.split_once(" ").unwrap();
    let depth = d.parse::<usize>().unwrap();
    let v = l2
        .split([' ', ','])
        .filter_map(|x| x.parse::<usize>().ok())
        .collect::<Vec<_>>();
    let target = (v[0], v[1]);
    // println!("depth:{}, target:{:?}", depth, target);

    // add map offset after target
    let grid_size = (target.1 + 1 + 50, target.0 + 1 + 50);
    let mut grid = Grid2D::new_empty(grid_size.0, grid_size.1, ' ');

    let mut gi = HashMap::new(); // geologic index
    let mut el = HashMap::new(); // erosion level

    gi.insert((0, 0), 0);
    el.insert((0, 0), 0);
    gi.insert((target.1, target.0), 0);
    el.insert((target.1, target.0), depth % 20183);

    for x in 1..=grid_size.1 {
        gi.insert((x, 0), x * 16807);
        el.insert((x, 0), (x * 16807 + depth) % 20183);
    }
    for y in 1..=grid_size.0 {
        gi.insert((0, y), y * 48271);
        el.insert((0, y), (y * 48271 + depth) % 20183);
    }
    for x in 0..=grid_size.1 {
        for y in 0..=grid_size.0 {
            // erosion level
            if !el.contains_key(&(x, y)) {
                let el1 = el.get(&(x - 1, y)).unwrap();
                let el2 = el.get(&(x, y - 1)).unwrap();
                gi.entry((x, y)).or_insert_with(|| el1 * el2);
                el.insert((x, y), ((el1 * el2) + depth) % 20183);
            }
        }
    }

    for x in 0..grid_size.1 {
        for y in 0..grid_size.0 {
            match el.get(&(x, y)).unwrap() % 3 {
                0 => grid.set_at((y, x), '.'), // rock
                1 => grid.set_at((y, x), '='), // water
                2 => grid.set_at((y, x), '|'), // tree
                _ => panic!(),
            }
        }
    }
    // grid.set_at((0, 0), 'M');
    // grid.set_at((target.1, target.0), 'T');

    // grid.print();

    let mut result = Vec::new();
    let end_node = Step {
        cell: (target.1, target.0),
        tool: Tool::Torch,
    };
    result.push(dijkstra(&grid.clone(), end_node));
    Some(*result.iter().min().unwrap())
}

#[derive(Ord, PartialEq, PartialOrd, Eq, Debug, Copy, Clone)]
enum Tool {
    Climb,
    Torch,
    None,
}

#[derive(Ord, PartialEq, PartialOrd, Eq, Debug, Copy, Clone)]
struct Step {
    cell: (usize, usize),
    tool: Tool,
}

fn dijkstra(
    grid: &Grid2D,
    end_node: Step,
    //dejavu: &mut Vec<bool>,
) -> usize {
    // on va obtenir une liste de noeuds et leur distance par rapport au point de départ
    let mut ans: BTreeMap<Step, (Step, usize)> = BTreeMap::new();
    // prec est la liste des points précédents
    let mut prec: BTreeMap<Step, usize> = BTreeMap::new();

    // les points de départ
    let start_node = Step {
        cell: (0, 0),
        tool: Tool::Torch,
    };
    ans.insert(start_node, (start_node, 0));
    prec.insert(start_node, 0);
    ans.insert(
        start_node,
        (
            Step {
                cell: start_node.cell,
                tool: Tool::Climb,
            },
            7,
        ),
    );
    prec.insert(
        Step {
            cell: start_node.cell,
            tool: Tool::Climb,
        },
        7,
    );

    // node -(cost)-> new_node
    while !prec.is_empty() {
        let (current_node, current_dist) = prec.pop_first().unwrap();
        // don't move but change tool
        match grid.get_at((current_node.cell.0, current_node.cell.1)) {
            '.' => {
                if current_node.tool == Tool::Climb {
                    let new_node = Step {
                        cell: current_node.cell,
                        tool: Tool::Torch,
                    };
                    dijkstra_insert(
                        current_node,
                        new_node,
                        current_dist + 7,
                        &mut ans,
                        &mut prec,
                    );
                } else if current_node.tool == Tool::Torch {
                    let new_node = Step {
                        cell: current_node.cell,
                        tool: Tool::Climb,
                    };
                    dijkstra_insert(
                        current_node,
                        new_node,
                        current_dist + 7,
                        &mut ans,
                        &mut prec,
                    );
                }
            }
            '=' => {
                if current_node.tool == Tool::Climb {
                    let new_node = Step {
                        cell: current_node.cell,
                        tool: Tool::None,
                    };
                    dijkstra_insert(
                        current_node,
                        new_node,
                        current_dist + 7,
                        &mut ans,
                        &mut prec,
                    );
                } else if current_node.tool == Tool::None {
                    let new_node = Step {
                        cell: current_node.cell,
                        tool: Tool::Climb,
                    };
                    dijkstra_insert(
                        current_node,
                        new_node,
                        current_dist + 7,
                        &mut ans,
                        &mut prec,
                    );
                }
            }
            '|' => {
                if current_node.tool == Tool::Torch {
                    let new_node = Step {
                        cell: current_node.cell,
                        tool: Tool::None,
                    };
                    dijkstra_insert(
                        current_node,
                        new_node,
                        current_dist + 7,
                        &mut ans,
                        &mut prec,
                    );
                } else if current_node.tool == Tool::None {
                    let new_node = Step {
                        cell: current_node.cell,
                        tool: Tool::Torch,
                    };
                    dijkstra_insert(
                        current_node,
                        new_node,
                        current_dist + 7,
                        &mut ans,
                        &mut prec,
                    );
                }
            }
            _ => panic!(),
        }
        // move without changing tool
        let adj = grid.get_adjacents_ortho(current_node.cell.0, current_node.cell.1);
        for a in adj {
            let new_pos = (a.0, a.1);
            match grid.get_at((new_pos.0, new_pos.1)) {
                '.' => {
                    if current_node.tool != Tool::None {
                        let new_node = Step {
                            cell: new_pos,
                            tool: current_node.tool,
                        };
                        dijkstra_insert(
                            current_node,
                            new_node,
                            current_dist + 1,
                            &mut ans,
                            &mut prec,
                        );
                    }
                }
                '=' => {
                    if current_node.tool != Tool::Torch {
                        let new_node = Step {
                            cell: new_pos,
                            tool: current_node.tool,
                        };
                        dijkstra_insert(
                            current_node,
                            new_node,
                            current_dist + 1,
                            &mut ans,
                            &mut prec,
                        );
                    }
                }
                '|' => {
                    if current_node.tool != Tool::Climb {
                        let new_node = Step {
                            cell: new_pos,
                            tool: current_node.tool,
                        };
                        dijkstra_insert(
                            current_node,
                            new_node,
                            current_dist + 1,
                            &mut ans,
                            &mut prec,
                        );
                    }
                }
                _ => {}
            }
        }
    }

    // for a in &ans {
    //     println!("{:?}", a);
    // }

    let mut path: Vec<(Step, usize)> = Vec::new();
    let mut path_vec = Vec::new();
    let mut node = end_node;
    while node != start_node {
        let d;
        (node, d) = *ans.get(&node).unwrap();
        path.insert(0, (node, d));
        path_vec.push(node.cell);
    }
    // println!("chemin le plus court :");
    // for p in path {
    //     println!("{:?}", p);
    // }
    // println!("{:?}", end_node);

    // grid.print();
    // grid.print_with_vec(&path_vec, 'X');

    let (_, dist) = ans.get(&end_node).unwrap();
    *dist
}

fn dijkstra_insert(
    current_node: Step,
    new_node: Step,
    new_cost: usize,
    ans: &mut BTreeMap<Step, (Step, usize)>,
    prec: &mut BTreeMap<Step, usize>,
) {
    if !ans.contains_key(&new_node) || new_cost < ans.get(&new_node).unwrap().1 {
        // println!("new best way to {:?} - cost = {}", new_node, new_cost);
        ans.insert(new_node, (current_node, new_cost));
        prec.insert(new_node, new_cost); // les points suivants à visiter + leur distance par rapport au départ
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_total() {
        assert_eq!(
            get_answer(include_str!("../assets/day22_input_demo1.txt")),
            Some(45)
        );
        assert_eq!(
            get_answer(include_str!("../assets/day22_input.txt")),
            Some(1054)
        );
    }
}
