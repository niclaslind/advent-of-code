fn file_to_vec() -> Vec<String> {
    include_str!("input/day4.txt")
        .split('\n')
        .map(|l| l.to_string())
        .collect()
}

fn read_fields(s: &str) -> bool {
    s.contains("byr:")
        && s.contains("iyr:")
        && s.contains("eyr:")
        && s.contains("hgt:")
        && s.contains("hcl:")
        && s.contains("ecl:")
        && s.contains("pid:")
}

pub fn part1() -> usize {
    include_str!("input/day4.txt")
        .split('\n')
        .filter(|s| read_fields(s))
        .count()
}

pub fn part2() -> i32 {
    file_to_vec();
    0
}

#[cfg(test)]
mod tests {
    use crate::day4::{part1, part2, read_fields};

    #[test]
    fn test_part1() {
        assert_eq!(part1(), 196)
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), 0)
    }

    #[test]
    fn test() {
        let input = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
                        byr:1937 iyr:2017 cid:147 hgt:183cm

                        iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
                        hcl:#cfa07d byr:1929

                        hcl:#ae17e1 iyr:2013
                        eyr:2024
                        ecl:brn pid:760753108 byr:1931
                        hgt:179cm

                        hcl:#cfa07d eyr:2025 pid:166559648
                        iyr:2011 ecl:brn hgt:59in"
            .split("\n\n")
            .filter(|s| read_fields(s))
            .count();
        assert_eq!(input, 2);
    }
}
