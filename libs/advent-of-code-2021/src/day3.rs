use itertools::Itertools;

pub fn part1() -> usize {
    let report = read_input();

    let mut freq: [u32; 12] = [0; 12];
    for bstr in &report {
        for (i, fx) in freq.iter_mut().rev().enumerate() {
            // Right shift the ith-bit to the lsb and isolate it
            *fx += (bstr >> i) & 1;
        }
    }
    let half = (report.len() / 2) as u32;
    let gr = freq.iter().rev().enumerate().fold(
        0,
        |sum, (i, fx)| if fx >= &half { sum + (1 << i) } else { sum },
    );
    gr * (gr ^ 0xFFF)
}

pub fn part2() -> u32 {
    let og_rating = find_rating(&read_input(), 0);
    let co2_rating = find_rating(&read_input(), 1);
    og_rating * co2_rating
}

fn find_rating(input: &[u32], mode: u32) -> u32 {
    let mut cand = input.to_vec();

    for bit_i in (0..12).rev() {
        if cand.len() == 1 {
            break;
        }

        let freq = cand.iter().fold(0, |sum, bx| sum + ((bx >> bit_i) & 1));
        let test_bit = (if freq >= (cand.len() as u32 - freq) {
            1
        } else {
            0
        }) ^ mode;

        cand = cand
            .into_iter()
            .filter(|bx| (*bx >> bit_i) & 1 == test_bit)
            .collect();
    }

    cand[0]
}

fn read_input() -> Vec<u32> {
    include_str!("input/day3.txt")
        .lines()
        .map(|line| u32::from_str_radix(line, 2).unwrap())
        .collect_vec()
}

#[cfg(test)]
mod tests {
    use crate::day3::{part1, part2};

    #[test]
    fn test_part1() {
        assert_eq!(part1(), 2035764);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), 2817661);
    }
}
