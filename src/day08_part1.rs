use std::{fmt, time::Instant};

// personal functions
//use crate::utils::grid2d;
//use crate::utils::pause;
//use crate::utils::math;

pub fn main() {
    println!("-- Advent of Code - Day 08 - Part 1 --");
    let now = Instant::now();

    // let input = include_str!("../assets/day08_input_demo1.txt");
    let input = include_str!("../assets/day08_input.txt");

    if let Some(answer) = get_answer(input) {
        println!("The answer is : {}", answer);
    } else {
        println!("No answer found");
    }
    let elapsed1 = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed1);
}

//

#[derive(Debug, Default, Clone)]
struct Node {
    start: usize,
    end: usize,
    chld: usize,
    m_len: usize,
    children: Vec<Node>,
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let ch = self.children.len();
        if self.end == 0 {
            write!(f, "{{{}-? : {},{}}}", self.start, self.chld, self.m_len)
        } else {
            write!(
                f,
                "{{{}-{} : {},{} and {} children}}",
                self.start, self.end, self.chld, self.m_len,ch
            )
        }
    }
}

fn get_answer(input: &str) -> Option<usize> {
    let mem: Vec<_> = input
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .collect();
    // println!("len is {}", mem.len());

    let mut first_node = Node {
        start: 0,
        end: mem.len() - 1,
        chld: mem[0],
        m_len: mem[1],
        children: Vec::new(),
    };
    // println!("first_node : {}", first_node);

    // find all children
    find_all_children(&mem, &mut first_node);

    Some(sum_metadata(&mem, first_node))
  
}

fn sum_metadata(mem: &[usize], node: Node) -> usize {
    let mut sum = 0;
    // print!("{} :", node);
    for x in (0..node.m_len).rev() {
        // print!("{} ", mem[node.end - x]);
        sum += mem[node.end - x];
    }
    // println!();
    for c in node.children {
        sum += sum_metadata(mem, c);
    }
    sum
}

fn find_all_children(mem: &[usize], parent: &mut Node) {

    // println!("-- children {} of {} : ", parent.children.len() + 1, parent);

    if parent.children.len() == parent.chld {
        // println!("all children are found");
        parent.end = parent.children.last().unwrap().end + parent.m_len;
        // println!("parent is now {}",parent);
        return;
    }

    // create a child with start pos
    let start = if parent.children.is_empty() {
        parent.start + 2
    } else {
        parent.children.last().unwrap().end + 1
    };
    let mut a_child = Node {
        start,
        chld: mem[start],
        m_len: mem[start + 1],
        ..Default::default()
    };
    // println!("seems to start like {}", a_child);

    if a_child.chld == 0 {
        // add child to parent
        // print!("{} has no child => ", a_child);
        a_child.end = a_child.start + 1 + a_child.m_len;
        parent.children.push(a_child.clone());
        // print!(" add {}", a_child);
        // println!(" to {}", parent);
    } else {
        // println!("has children => find next child");
        find_all_children(mem, &mut a_child);
        parent.children.push(a_child.clone());
    }

    find_all_children(mem, parent);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_total() {
        assert_eq!(
            get_answer(include_str!("../assets/day08_input_demo1.txt")),
            Some(138)
        );
        assert_eq!(get_answer(include_str!("../assets/day08_input.txt")), Some(40984));
    }
}
