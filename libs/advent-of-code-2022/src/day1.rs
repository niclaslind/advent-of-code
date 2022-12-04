pub fn count_calories(count: usize) -> u32 {
    let mut cals = include_str!("input/day1.txt")
        .split("\n\n")
        .map(|e| e.lines().map(|c| c.parse::<u32>().unwrap()).sum::<u32>())
        .collect::<Vec<u32>>();

    cals.sort_unstable();
    cals.into_iter().rev().take(count).sum::<u32>()
}

#[cfg(test)]
mod tests {
    use super::count_calories;

    #[test]
    fn test_part1() {
        assert_eq!(69310, count_calories(1));
    }

    #[test]
    fn test_part2() {
        assert_eq!(206104, count_calories(3));
    }
}
