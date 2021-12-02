pub fn part1() -> u32 {
    let (forward, depth) = include_str!("input/day2.txt")
        .lines()
        .filter_map(|s| {
            s.split_once(" ")
                .map(|(dir, val)| (dir, val.parse::<u32>().unwrap()))
        })
        .fold((0, 0), |(x, y), (dir, val)| match dir {
            "forward" => (x + val, y),
            "down" => (x, y + val),
            "up" => (x, y - val),
            _ => unreachable!(),
        });

    forward * depth
}

pub fn part2() -> u32 {
    let (forward, depth, _) = include_str!("input/day2.txt")
    .lines()
    .filter_map(|s| {
            s.split_once(" ")
                .map(|(dir, val)| (dir, val.parse::<u32>().unwrap()))
        })
        .fold((0, 0, 0), |(x, y, aim), (dir, val)| match dir {
            "forward" => (x + val, y + (aim * val), aim),
            "down" => (x, y, aim + val),
            "up" => (x, y, aim - val),
            _ => unreachable!(),
        });

    forward * depth
}

#[cfg(test)]
mod tests {
    use crate::day2::{part1, part2};

    #[test]
    fn test_part1() {
        assert_eq!(2117664, part1());
    }

    #[test]
    fn test_part2() {
        assert_eq!(2073416724, part2());
    }
}
