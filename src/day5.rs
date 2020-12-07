use std::io::{BufReader, BufRead};
use std::fs;

fn file_to_vec() -> Vec<String> {
    let file_in = fs::File::open("src/input/day5.txt".to_string()).expect("Could not found file");
    let file_reader = BufReader::new(file_in);
    let lines: Vec<String> = file_reader.lines().into_iter().map(|l| {
        l.unwrap()
    }).collect();
    lines
}

fn get_seat_id(code: &String) -> String {
    let seat: String = code.chars().map(|token| match token {
        'B' | 'R' => '1',
        'F' | 'L' => '0',
        _ => token
    }).collect();
    seat
}

fn get_all_seats_ids() -> Vec<i32> {
    let seats = file_to_vec();
    let mut seat_ids: Vec<i32> = Vec::new();

    for (_index, st) in seats.iter().enumerate() {
        let seat_id = i32::from_str_radix(&get_seat_id(st), 2).unwrap();
        seat_ids.push(seat_id)
    }
    seat_ids.sort_by(|a, b| b.cmp(a));
    seat_ids
}

fn part1() {
    let seat_ids = get_all_seats_ids();
    println!("Part1 - {}", seat_ids[0]);
}

fn part2() {
    let seat_ids = get_all_seats_ids();
    let min = seat_ids[seat_ids.len() - 1];
    let max = seat_ids[0];
    for i in min..(max) {
        if !seat_ids.contains(&i) {
            println!("Part2 - {}", i);
        }
    }
}

pub fn main() {
    part1();
    part2();
}
