use std::fs;

use scan_fmt::scan_fmt;

fn file_to_vec() -> Vec<String> {
    fs::read_to_string("src/input/day2.txt")
        .expect("Could not read file")
        .split('\n')
        .map(|l| l.to_string())
        .collect()
}

fn check_password_valid(s: &str) -> bool {
    let (min, max, c, password) =
        scan_fmt!(s, "{d}-{d} {}: {}", usize, usize, char, String).unwrap();
    let sum = password.chars().filter(|token| token == &c).count();
    sum >= min && sum <= max
}

fn check_password_valid2(s: &str) -> bool {
    let (min, max, c, password) =
        scan_fmt!(s, "{d}-{d} {}: {}", usize, usize, char, String).unwrap();
    let sum1 = password.chars().nth(min - 1).unwrap() == c;
    let sum2 = password.chars().nth(max - 1).unwrap() == c;
    (sum1 | sum2) ^ (sum1 & sum2)
}

fn part1() -> usize {
    file_to_vec()
        .iter()
        .filter(|s| check_password_valid(s))
        .count()
}

fn part2() -> usize {
    file_to_vec()
        .iter()
        .filter(|s| check_password_valid2(s))
        .count()
}

#[cfg(test)]
mod tests {
    use crate::day2::{part1, part2};

    #[test]
    fn test_part1() {
        assert_eq!(part1(), 614)
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), 354)
    }
}
