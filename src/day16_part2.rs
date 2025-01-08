use std::{
    collections::{HashMap, HashSet},
    time::Instant,
};

// personal functions
//use crate::utils::grid2d;
//use crate::utils::pause;
//use crate::utils::math;

pub fn main() {
    println!("-- Advent of Code - Day 16 - Part 2 --");
    let now = Instant::now();

    let input = include_str!("../assets/day16_input.txt");

    if let Some(answer) = get_answer(input) {
        println!("The answer is : {}", answer);
    } else {
        println!("No answer found");
    }
    let elapsed1 = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed1);
}

// 637 too high

#[derive(Default, Debug, Clone, Copy)]
struct Instruction {
    opcode: Opcode,
    a: usize,
    b: usize,
    c: usize,
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Opcode {
    #[default]
    Addr,
    Addi,
    Mulr,
    Muli,
    Banr,
    Bani,
    Borr,
    Bori,
    Setr,
    Seti,
    Gtir,
    Gtri,
    Gtrr,
    Eqir,
    Eqri,
    Eqrr,
}

fn get_answer(input: &str) -> Option<usize> {
    // let mut result = 0;
    let mut enum_values: HashMap<usize, HashSet<Opcode>> = HashMap::new();
    let mut current = 0;

    let mut registers_before: Vec<usize> = Vec::new();
    let mut registers_after: Vec<usize> = Vec::new();
    let mut instruction = Instruction {
        ..Default::default()
    };

    for (i, lines) in input.lines().enumerate() {
        match i % 4 {
            0 => {
                if !lines.starts_with("Before:") {
                    break;
                }
                registers_before = lines
                    .split(['[', ' ', ',', ']'])
                    .filter_map(|x| x.parse::<usize>().ok())
                    .collect();
                if registers_before.len() != 4 {
                    panic!();
                }
            }
            1 => {
                let x: Vec<_> = lines
                    .split_whitespace()
                    .filter_map(|x| x.parse::<usize>().ok())
                    .collect();
                instruction = Instruction {
                    a: x[1],
                    b: x[2],
                    c: x[3],
                    ..Default::default()
                };
                current = x[0];
                enum_values.insert(
                    x[0] ,
                    HashSet::from([
                        Opcode::Addr,
                        Opcode::Addi,
                        Opcode::Mulr,
                        Opcode::Muli,
                        Opcode::Banr,
                        Opcode::Bani,
                        Opcode::Borr,
                        Opcode::Bori,
                        Opcode::Setr,
                        Opcode::Seti,
                        Opcode::Gtir,
                        Opcode::Gtri,
                        Opcode::Gtrr,
                        Opcode::Eqir,
                        Opcode::Eqri,
                        Opcode::Eqrr,
                    ]),
                );
            }
            2 => {
                if !lines.starts_with("After:") {
                    break;
                }
                registers_after = lines
                    .split(['[', ' ', ',', ']'])
                    .filter_map(|x| x.parse::<usize>().ok())
                    .collect();
                if registers_after.len() != 4 {
                    panic!();
                }
            }
            _ => {
                // let mut fits = 0;
                // println!("----");
                // println!("before = {:?}", registers_before);
                // println!("instructions = {:?}", instruction);
                // println!("after = {:?}", registers_after);
                for opcode in [
                    Opcode::Addr,
                    Opcode::Addi,
                    Opcode::Mulr,
                    Opcode::Muli,
                    Opcode::Banr,
                    Opcode::Bani,
                    Opcode::Borr,
                    Opcode::Bori,
                    Opcode::Setr,
                    Opcode::Seti,
                    Opcode::Gtir,
                    Opcode::Gtri,
                    Opcode::Gtrr,
                    Opcode::Eqir,
                    Opcode::Eqri,
                    Opcode::Eqrr,
                ] {
                    let mut instruction2 = instruction;
                    instruction2.opcode = opcode;
                    let mut registers2 = registers_before.clone();
                    do_opcode(&mut registers2, instruction2);
                    if registers2 == registers_after {
                        // println!("{:?} could fit", instruction2);
                        // fits += 1;
                    } else if let Some(x) = enum_values.get(&(current )) {
                        let mut z = x.clone();
                        z.remove(&opcode);
                        enum_values.insert(current , z);
                    }
                }
                // if fits >= 3 {
                //     result += 1;
                // }
            }
        }
    }

    // check values
    for _ in 0..=15 {
        for i in 0..=15 {
            if enum_values.get(&i).unwrap().len() == 1 {
                let op = *enum_values.clone().get(&i).unwrap().iter().next().unwrap();
                // println!("{} is {:?}", i, op);
                for j in 0..=15 {
                    if i != j {
                        if let Some(x) = enum_values.get(&(j)) {
                            let mut z = x.clone();
                            z.remove(&op);
                            enum_values.insert(j, z);
                        }
                    }
                }
            }
        }
    }
    // for i in 0..=15 {
    //     println!("{i} is {:?}", enum_values.get(&i).unwrap());
    // }

    // final program
    let mut registers: Vec<usize> = vec![0,0,0,0];
    let (_,prog) = input.split_once("\n\n\n\n").unwrap();

    prog.lines().for_each(|line| {
        // println!("line : {:?}",line);
        let x: Vec<_> = line
                    .split_whitespace()
                    .filter_map(|x| x.parse::<usize>().ok())
                    .collect();
        instruction = Instruction {
                    opcode : *enum_values.clone().get(&(x[0] )).unwrap().iter().next().unwrap(),
                    a: x[1],
                    b: x[2],
                    c: x[3],
                };
        do_opcode(&mut registers, instruction);

    });

    Some(registers[0] )
}

fn do_opcode(register: &mut [usize], instruction: Instruction) {
    // println!("in do_opcode({:?},{:?})",register,instruction);
    let c = match instruction.opcode {
        Opcode::Addr => register[instruction.a ] + register[instruction.b ],
        Opcode::Addi => register[instruction.a ] + instruction.b,
        Opcode::Mulr => register[instruction.a ] * register[instruction.b ],
        Opcode::Muli => register[instruction.a ] * instruction.b,
        Opcode::Banr => register[instruction.a ] & register[instruction.b ],
        Opcode::Bani => register[instruction.a ] & instruction.b,
        Opcode::Borr => register[instruction.a ] | register[instruction.b ],
        Opcode::Bori => register[instruction.a ] | instruction.b,
        Opcode::Setr => register[instruction.a ],
        Opcode::Seti => instruction.a,

        Opcode::Gtir => {
            if instruction.a > register[instruction.b ] {
                1
            } else {
                0
            }
        }
        Opcode::Gtri => {
            if register[instruction.a ] > instruction.b {
                1
            } else {
                0
            }
        }
        Opcode::Gtrr => {
            if register[instruction.a ] > register[instruction.b ] {
                1
            } else {
                0
            }
        }

        Opcode::Eqir => {
            if instruction.a == register[instruction.b ] {
                1
            } else {
                0
            }
        }
        Opcode::Eqri => {
            if register[instruction.a ] == instruction.b {
                1
            } else {
                0
            }
        }
        Opcode::Eqrr => {
            if register[instruction.a ] == register[instruction.b ] {
                1
            } else {
                0
            }
        }
    };
    // println!("c = {}", c);
    register[instruction.c ] = c;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_total() {
        assert_eq!(
            get_answer(include_str!("../assets/day16_input.txt")),
            Some(653)
        );
    }
}
