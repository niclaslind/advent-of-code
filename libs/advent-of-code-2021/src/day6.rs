use std::collections::VecDeque;

use nom::{bytes::complete::tag, character::complete::digit1, multi::separated_list1, IResult};

pub fn process(input: &str, days: u32) -> u64 {
    let (_, mut fishes) = read_puzzle_input(input).unwrap();
    for _ in 0..days {
        fishes.rotate_left(1);

        let new_fishes = *fishes.get(8).unwrap();
        let old_fishes = fishes.get_mut(6).unwrap();
        *old_fishes += new_fishes;
    }
    fishes.iter().sum()
}

fn read_puzzle_input(input: &str) -> IResult<&str, VecDeque<u64>> {
    let (input, fish) = separated_list1(tag(","), digit1)(input)?;
    let mut arr = [0; 9];

    for num in fish.iter() {
        let num: usize = num.parse().unwrap();
        arr[num] += 1;
    }

    let que = VecDeque::from(arr);
    Ok((input, que))
}

pub fn part1() -> u64 {
    process(include_str!("./input/day6.txt"), 80)
}

pub fn part2() -> u64 {
    process(include_str!("./input/day6.txt"), 256)
}

#[cfg(test)]
pub mod tests {
    use crate::day6::{part1, part2};

    #[test]
    fn test_part1() {
        assert_eq!(part1(), 393019);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), 1757714216975)
    }
}
