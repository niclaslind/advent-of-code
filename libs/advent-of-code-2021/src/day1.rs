pub fn part1() -> i32 {
    let measurments = read_measurments();

    measurments
        .iter()
        .enumerate()
        .skip(1)
        .filter(|(index, &value)| value > measurments[index - 1])
        .count() as i32
}

pub fn part2() -> i32 {
    let measurments = read_measurments();

    measurments
        .iter()
        .enumerate()
        .skip(3)
        .filter(|(index, &value)| {
            let latest_sum =
                measurments[index - 3] + measurments[index - 2] + measurments[index - 1];
            let new_sum = measurments[index - 2] + measurments[index - 1] + value;

            new_sum > latest_sum
        })
        .count() as i32
}

fn read_measurments() -> Vec<i32> {
    include_str!("input/day1.txt")
        .split('\n')
        .map(|value| value.parse::<i32>().unwrap())
        .collect::<Vec<i32>>()
}

#[cfg(test)]
mod tests {
    use crate::day1::{part1, part2};

    #[test]
    fn test_part1() {
        assert_eq!(1228, part1());
    }

    #[test]
    fn test_part2() {
        assert_eq!(1257, part2());
    }
}
