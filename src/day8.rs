use std::collections::{HashMap, HashSet};
use std::ops::Index;
use crate::day8::Instruction::{Acc, Jmp, Nop};
use itertools::Itertools;
use std::mem::replace;
use std::fs;

pub fn main() {
    let mut instructions = parse_instructions(include_str!("input/day8.txt"));
    println!("Part1 - {:?}", part1(instructions.clone()));
    println!("Part2 - {:?}", part2(instructions.clone()));
}

#[derive(Clone, Debug, Copy)]
enum Instruction {
    Acc(i32),
    Jmp(i32),
    Nop(i32),
}

struct Commandore {
    instructions: Vec<Instruction>,
    instruction_line: usize,
    acc: i32,
}

impl Commandore {
    fn new(instructions: Vec<Instruction>) -> Self {
        Self {
            instructions,
            instruction_line: 0,
            acc: 0,
        }
    }

    fn step(&mut self) -> bool {
        if self.instruction_line > self.instructions.len() {
            return false;
        }

        match self.instructions[self.instruction_line] {
            Instruction::Nop { .. } => {}
            Instruction::Acc(n) => { self.acc += n; }
            Instruction::Jmp(n) => {
                self.instruction_line = (self.instruction_line as i32 + n) as usize;
                return true;
            }
        }
        self.instruction_line += 1;

        true
    }

    fn acc(&self) -> i32 {
        self.acc
    }

    fn run(&mut self) -> bool {
        let mut set = HashSet::new();
        loop {
            if set.insert(self.instruction_line) {
                if !self.step() {
                    return true;
                }
            } else {
                return false;
            }
        }
    }
}

fn parse_instructions(input: &str) -> Vec<Instruction> {
    input.lines().map(|instruction| {
        let (name, value) = scan_fmt!(instruction, "{} {d}", String, i32).unwrap();
        match name.as_str() {
            "acc" => Acc(value),
            "jmp" => Jmp(value),
            "nop" => Nop(value),
            _ => panic!()
        }
    }).collect()
}


// 1446
fn part1(mut instructions: Vec<Instruction>) -> i32 {
    let mut commandore = Commandore::new(instructions);
    commandore.run();
    commandore.acc()
}

// 1403
fn part2(mut instructions: Vec<Instruction>) -> i32 {
    // for i in 0..instructions.len() {
    //     let mut instructions = instructions.clone();
    //     instructions[i] = convert_instruction(instructions[i]);
    //     match run_machine(instructions) {
    //         Success(n) => return Success(n),
    //         _ => ()
    //     }
    // }
    0
}

fn convert_instruction(instruction: Instruction) -> Instruction {
    return match instruction {
        Instruction::Jmp(n) => Instruction::Nop(n),
        Instruction::Nop(n) => Instruction::Jmp(n),
        c => c
    };
}

#[test]
fn test_example() {
    let mut instructions =
        parse_instructions(include_str!("example_input/day8.txt"));
    // assert_eq!(part1(instructions), Success(5))
}