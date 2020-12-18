use std::{fs};
use itertools::Itertools;

fn file_to_vec() -> Vec<i32> {
    fs::read_to_string("src/input/day1.txt")
        .expect("Failed to read input")
        .split_whitespace()
        .map(|s| s.parse().unwrap_or(0))
        .collect()
}

fn part1() -> i32 {
    let numbers = file_to_vec();
    let mut sum: i32 = 0;
    for (first, second) in numbers.iter().tuple_combinations() {
        if first + second == 2020 {
            sum = first * second;
            println!("Part1 - {}", sum);
        }
    }
    sum
}

fn part2() -> i32 {
    let mut sum: i32 = 0;
    for (first, second, third) in file_to_vec().iter().tuple_combinations() {
        if first + second + third == 2020 {
            sum = first * second * third;
            println!("Part2 - {}", sum);
        }
    }
    sum
}

pub fn main() {
    part1();
    part2();
}

#[test]
fn test_part1() {
    assert_eq!(part1(), 1003971)
}

#[test]
fn test_part2() {
    assert_eq!(part2(), 84035952)
}