use std::io::{BufReader, BufRead};
use std::fs;

fn file_to_vec() -> Vec<String> {
    let file_in = fs::File::open("src/input/day2.txt".to_string()).expect("Could not found file");
    let file_reader = BufReader::new(file_in);
    file_reader
        .lines()
        .map(|l| l.unwrap())
        .collect()
}

fn check_password_valid(s: &String) -> bool {
    let (min, max, c, password)
        = scan_fmt!(s, "{d}-{d} {}: {}", usize, usize, char, String)
        .unwrap();

    let sum = password.chars().filter(|token| token == &c).count();
    sum >= min && sum <= max
}

fn check_password_valid2(s: &String) -> bool {
    let (min, max, c, password)
        = scan_fmt!(s, "{d}-{d} {}: {}", usize, usize, char, String)
        .unwrap();

    let sum1 = password.chars().nth(min - 1).unwrap() == c;
    let sum2 = password.chars().nth(max - 1).unwrap() == c;
    (sum1 | sum2) ^ (sum1 & sum2)
}

fn part1() {
    let passwords = file_to_vec();
    let valid_passwords = passwords
        .iter()
        .filter(|s| check_password_valid(s))
        .count();
    println!("Part1 - {}", valid_passwords);
}

fn part2() {
    let passwords = file_to_vec();
    let valid_password = passwords
        .iter()
        .filter(|s| check_password_valid2(s))
        .count();
    println!("Part2 - {}", valid_password);
}

pub fn main() {
    part1();
    part2();
}