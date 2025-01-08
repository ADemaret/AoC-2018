use std::{fmt, time::Instant};

// personal functions
//use crate::utils::grid2d;
//use crate::utils::pause;
//use crate::utils::math;

pub fn main() {
    println!("-- Advent of Code - Day 19 - Part 2 --");
    let now = Instant::now();

    // let input = include_str!("../assets/day19_input_demo1.txt");
    let input = include_str!("../assets/day19_input.txt");

    if let Some(answer) = get_answer(input) {
        println!("The answer is : {}", answer);
    } else {
        println!("No answer found");
    }
    let elapsed1 = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed1);
}

//

#[derive(Default, Debug, Clone, Copy)]
struct Instruction {
    opcode: Opcode,
    a: usize,
    b: usize,
    c: usize,
}

impl fmt::Display for Instruction{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?} {} {} {}", self.opcode, self.a,self.b,self.c)
    }
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
    let (ip_line, prog) = input.split_once("\n").unwrap();
    let (_, ip_str) = ip_line.split_once(" ").unwrap();
    let mut ip = ip_str.parse::<usize>().unwrap();
    let ip_reg = ip;
    ip = 0;
    println!("pointer : {ip}");

    let mut registers: Vec<usize> = vec![1, 0, 0, 0, 0, 0];

    let program = prog
        .lines()
        .map(|line| {
            // println!("line : {:?}",line);
            let x: Vec<_> = line.split_whitespace().collect();
            Instruction {
                opcode: match x[0] {
                    "addr" => Opcode::Addr,
                    "addi" => Opcode::Addi,
                    "mulr" => Opcode::Mulr,
                    "muli" => Opcode::Muli,
                    "banr" => Opcode::Banr,
                    "bani" => Opcode::Bani,
                    "borr" => Opcode::Borr,
                    "bori" => Opcode::Bori,
                    "setr" => Opcode::Setr,
                    "seti" => Opcode::Seti,
                    "gtir" => Opcode::Gtir,
                    "gtri" => Opcode::Gtri,
                    "gtrr" => Opcode::Gtrr,
                    "eqir" => Opcode::Eqir,
                    "eqri" => Opcode::Eqri,
                    "eqrr" => Opcode::Eqrr,
                    _ => panic!(),
                },
                a: x[1].parse::<usize>().unwrap(),
                b: x[2].parse::<usize>().unwrap(),
                c: x[3].parse::<usize>().unwrap(),
            }
        })
        .collect::<Vec<Instruction>>();

    for (i,p) in program.iter().enumerate() {
        println!("{i} : {:?}",p);
    }

    loop {
        registers[ip_reg] = ip;
        // print!("ip={ip} {:?} {}",registers,program[ip]);
        do_opcode(&mut registers, program[ip]);
        println!(" {:?}",registers);
        ip = registers[ip_reg];
        ip+=1;
        if ip >= program.len() {
            break;
        }
    }

    Some(registers[0])
}

fn do_opcode(register: &mut [usize], instruction: Instruction) {
    // println!("in do_opcode({:?},{:?})",register,instruction);
    let c = match instruction.opcode {
        Opcode::Addr => register[instruction.a] + register[instruction.b],
        Opcode::Addi => register[instruction.a] + instruction.b,
        Opcode::Mulr => register[instruction.a] * register[instruction.b],
        Opcode::Muli => register[instruction.a] * instruction.b,
        Opcode::Banr => register[instruction.a] & register[instruction.b],
        Opcode::Bani => register[instruction.a] & instruction.b,
        Opcode::Borr => register[instruction.a] | register[instruction.b],
        Opcode::Bori => register[instruction.a] | instruction.b,
        Opcode::Setr => register[instruction.a],
        Opcode::Seti => instruction.a,

        Opcode::Gtir => {
            if instruction.a > register[instruction.b] {
                1
            } else {
                0
            }
        }
        Opcode::Gtri => {
            if register[instruction.a] > instruction.b {
                1
            } else {
                0
            }
        }
        Opcode::Gtrr => {
            if register[instruction.a] > register[instruction.b] {
                1
            } else {
                0
            }
        }

        Opcode::Eqir => {
            if instruction.a == register[instruction.b] {
                1
            } else {
                0
            }
        }
        Opcode::Eqri => {
            if register[instruction.a] == instruction.b {
                1
            } else {
                0
            }
        }
        Opcode::Eqrr => {
            if register[instruction.a] == register[instruction.b] {
                1
            } else {
                0
            }
        }
    };
    // println!("c = {}", c);
    register[instruction.c] = c;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_total() {
        assert_eq!(
            get_answer(include_str!("../assets/day19_input_demo1.txt")),
            Some(6)
        );
        // assert_eq!(get_answer(include_str!("../assets/day19_input.txt")), None);
    }
}
