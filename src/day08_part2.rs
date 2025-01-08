use std::{fmt, time::Instant};

// personal functions
//use crate::utils::grid2d;
//use crate::utils::pause;
//use crate::utils::math;

pub fn main() {
    println!("-- Advent of Code - Day 08 - Part 2 --");
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
    value: usize,
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let ch = self.children.len();
        if self.end == 0 {
            write!(
                f,
                "{{{}-? : {},{} = {}}}",
                self.start, self.chld, self.m_len, self.value
            )
        } else {
            write!(
                f,
                "{{{}-{} : {},{} and {} children, value = {}}}",
                self.start, self.end, self.chld, self.m_len, ch, self.value
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
        value: 0,
    };
    // println!("first_node : {}", first_node);

    // find all children
    find_all_children(&mem, &mut first_node);

    Some(first_node.value)
}

fn find_all_children(mem: &[usize], parent: &mut Node) {
    // println!("-- children {} of {} : ", parent.children.len() + 1, parent);

    if parent.children.len() == parent.chld {
        // println!("all children are found");
        parent.end = parent.children.last().unwrap().end + parent.m_len;
        for i in (0..parent.m_len).rev() {
            let index = mem[parent.end - i];
            if index - 1 < parent.children.len() {
                parent.value += parent.children[index - 1].value;
            }
        }
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
        for m in 0..a_child.m_len {
            a_child.value += mem[a_child.end - m];
        }
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
            Some(66)
        );
        assert_eq!(
            get_answer(include_str!("../assets/day08_input.txt")),
            Some(37067)
        );
    }
}
