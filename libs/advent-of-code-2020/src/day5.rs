use std::cmp::Reverse;
use std::fs;
use std::ops::Index;

fn file_to_vec() -> Vec<String> {
    fs::read_to_string("src/input/day5.txt")
        .expect("Could not read file")
        .split('\n')
        .map(|l| l.to_string())
        .collect()
}

fn get_seat_id(code: &str) -> String {
    code.chars()
        .map(|token| match token {
            'B' | 'R' => '1',
            'F' | 'L' => '0',
            _ => token,
        })
        .collect()
}

fn get_all_seats_ids() -> Vec<i32> {
    let seats = file_to_vec();
    let mut seat_ids: Vec<i32> = Vec::new();

    for (_index, st) in seats.iter().enumerate() {
        let seat_id = i32::from_str_radix(&get_seat_id(st), 2).unwrap();
        seat_ids.push(seat_id)
    }
    seat_ids.sort_by_key(|&b| Reverse(b));
    seat_ids
}

fn part1() -> i32 {
    *get_all_seats_ids().index(0)
}

fn part2() -> i32 {
    let seat_ids = get_all_seats_ids();
    let min = seat_ids[seat_ids.len() - 1];
    let max = seat_ids[0];
    for i in min..(max) {
        if !seat_ids.contains(&i) {
            return i;
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use crate::day5::{part1, part2};

    #[test]
    fn test_part1() {
        assert_eq!(part1(), 922)
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), 747)
    }
}
