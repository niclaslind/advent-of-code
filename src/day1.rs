use std::io::{BufReader, BufRead};
use std::{fs};
use itertools::Itertools;

fn file_to_vec() -> Vec<u32> {
    let file_in = fs::File::open("src/input/day1.txt".to_string()).expect("Could not found file");
    let file_reader = BufReader::new(file_in);
    let nums: Vec<u32> = file_reader.lines().into_iter().map(|l| {
        l.ok().and_then(|s| s.parse().ok()).unwrap_or(0)
    }).filter(|x| *x < 2020).collect();
    nums
}

fn part1() {
    let numbers = file_to_vec();

    for (first, second) in numbers.iter().tuple_combinations() {
        if first + second == 2020 {
            let sum = first * second;
            println!("Part1 - {}", sum);
        }
    }
}

fn part2() {
    let numbers = file_to_vec();

    for (first, second, third) in numbers.iter().tuple_combinations() {
        if first + second + third == 2020 {
            let sum = first * second * third;
            println!("Part2 - {}", sum);
        }
    }
}


pub fn main() {
    part1();
    part2();
}