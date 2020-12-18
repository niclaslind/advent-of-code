use std::fs;
use std::collections::HashMap;
use itertools::Itertools;

fn file_to_vec() -> Vec<String> {
    fs::read_to_string("src/input/day4.txt")
        .expect("Could not read file")
        .split("\n")
        .map(|l| l.to_string())
        .collect()
}


fn part1() -> i32 {
    let mut valid_passport = 0;
    file_to_vec().iter().filter(|c| !c.is_empty()).for_each(|x| {
        x.split(' ').for_each(|field| {
            let mut fields = field.split(":");
            let key = fields.next().unwrap();
            let value = fields.next().unwrap();
            println!("{} {} ", key, value);
            match key {
                "byr" => println!("byr - {}", value),
                "iyr" => println!("iyr - {}", value),
                "eyr" => println!("eyr - {}", value),
                "hgt" => println!("hgt - {}", value),
                "hcl" => println!("hcl - {}", value),
                "ecl" => println!("ecl - {}", value),
                "pid" => println!("pid - {}", value),
                "cid" => println!("cid - {}", value),
                _ => {}
            }
        })
    });
    valid_passport
}

fn part2() -> i32 {
    file_to_vec();
    0
}

pub fn main() {
    println!("Part1 {}", part1());
    println!("Part2 {}", part2());
}

#[test]
fn test_part1() {
    assert_eq!(part1(), 0)
}

#[test]
fn test_part2() {
    assert_eq!(part2(), 0)
}