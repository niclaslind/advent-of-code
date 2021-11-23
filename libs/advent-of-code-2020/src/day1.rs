use itertools::Itertools;

fn file_to_vec() -> Vec<i32> {
    include_str!("input/day1.txt")
        .split_whitespace()
        .map(|s| s.parse().unwrap_or(0))
        .collect()
}

pub fn part1() -> i32 {
    let numbers = file_to_vec();
    let mut sum: i32 = 0;
    for (first, second) in numbers.iter().tuple_combinations() {
        if first + second == 2020 {
            sum = first * second;
        }
    }
    sum
}

pub fn part2() -> i32 {
    let numbers = file_to_vec();
    let mut sum: i32 = 0;
    for (first, second, third) in numbers.iter().tuple_combinations() {
        if first + second + third == 2020 {
            sum = first * second * third;
        }
    }
    sum
}

#[cfg(test)]
mod tests {
    use crate::day1::{part1, part2};

    #[test]
    fn test_part1() {
        assert_eq!(part1(), 1003971)
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), 84035952)
    }
}
