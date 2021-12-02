pub fn count_increasing_measurments(window_size: usize) -> usize {
    let measurments: Vec<i32> = include_str!("input/day1.txt")
        .lines()
        .map(|val| val.parse::<i32>().unwrap())
        .collect();

    let sums: Vec<i32> = measurments
        .windows(window_size)
        .map(|window| window.iter().sum())
        .collect();

    sums.windows(2)
        .filter(|measurments| measurments[0] < measurments[1])
        .count()
}

#[cfg(test)]
mod tests {
    use crate::day1::count_increasing_measurments;

    #[test]
    fn test_part1() {
        assert_eq!(1228, count_increasing_measurments(1));
    }

    #[test]
    fn test_part2() {
        assert_eq!(1257, count_increasing_measurments(3));
    }
}
