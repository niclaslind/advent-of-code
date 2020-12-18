use std::fs;

fn file_to_vec() -> Vec<String> {
    fs::read_to_string("src/input/day3.txt")
        .expect("Could not read file")
        .split("\n")
        .map(|l| l.to_string())
        .collect()
}

fn part1() -> u64 {
    ride(&file_to_vec(), 3, 1)
}

fn part2() -> u64 {
    let map = file_to_vec();
    ride(&map, 3, 1) *
        ride(&map, 1, 1) *
        ride(&map, 5, 1) *
        ride(&map, 7, 1) *
        ride(&map, 1, 2)
}

fn ride(map: &Vec<String>, right_move: usize, down_mode: usize) -> u64 {
    let mut current_position = 0;
    let mut trees = 0;

    for (column_index, line) in map.iter().enumerate() {
        if column_index % down_mode == 0 {
            let row_index = current_position % line.chars().count();
            if line.chars().nth(row_index).unwrap() == '#' {
                trees += 1;
            }
            current_position += right_move;
        }
    }

    trees
}

pub fn main() {
    println!("Part1 - {}", part1());
    println!("Part2 - {}", part2());
}

#[test]
fn test_part1() {
    assert_eq!(part1(), 278)
}

#[test]
fn test_part2() {
    assert_eq!(part2(), 9709761600)
}