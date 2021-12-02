use regex::Regex;

#[derive(Default, Debug)]
struct Passport {
    byr: String,
    iyr: String,
    eyr: String,
    hgt: String,
    hcl: String,
    ecl: String,
    pid: String,
    cid: Option<String>,
}

impl Passport {
    fn new(s: &str) -> Option<Passport> {
        Self::all_fields_present(s).and_then(Self::check_validity)
    }

    pub fn all_fields_present(s: &str) -> Option<Passport> {
        let mut passport = Passport::default();

        s.split_whitespace().for_each(|field| {
            let parts = field
                .split_once(':')
                .map(|(field, value)| (field, value.to_string()));

            if let Some((field, value)) = parts {
                match field {
                    "byr" => passport.byr = value,
                    "iyr" => passport.iyr = value,
                    "eyr" => passport.eyr = value,
                    "hgt" => passport.hgt = value,
                    "hcl" => passport.hcl = value,
                    "ecl" => passport.ecl = value,
                    "pid" => passport.pid = value,
                    "cid" => passport.cid = Some(value),
                    _ => unreachable!(),
                }
            }
        });

        if !passport.byr.is_empty()
            && !passport.iyr.is_empty()
            && !passport.eyr.is_empty()
            && !passport.hgt.is_empty()
            && !passport.hcl.is_empty()
            && !passport.ecl.is_empty()
            && !passport.pid.is_empty()
        {
            Some(passport)
        } else {
            None
        }
    }

    fn check_validity(self) -> Option<Passport> {
        if check_pid(&self.pid)
            && check_byr(&self.byr)
            && check_iyr(&self.iyr)
            && check_eyr(&self.eyr)
            && check_hgt(&self.hgt)
            && check_hcl(&self.hcl)
            && check_ecl(&self.ecl)
        {
            Some(self)
        } else {
            None
        }
    }
}

fn check_byr(s: &str) -> bool {
    matches!(s.parse::<usize>(), Ok(v) if (1920..=2002).contains(&v))
}

fn check_iyr(s: &str) -> bool {
    match s.parse::<usize>() {
        Ok(v) => (2010..=2020).contains(&v),
        _ => false,
    }
}

fn check_eyr(s: &str) -> bool {
    match s.parse::<usize>() {
        Ok(v) => (2020..=2030).contains(&v),
        _ => false,
    }
}

fn check_hgt(s: &str) -> bool {
    let re = Regex::new(r"^(\d*)(in|cm)$").unwrap();
    match re.captures(s) {
        Some(cap) => {
            let val = cap[1].parse::<usize>().unwrap_or(0);
            let unit = &cap[2];
            match unit {
                "cm" => (150..=193).contains(&val),
                "in" => (59..=76).contains(&val),
                _ => false,
            }
        }
        None => false,
    }
}

fn check_hcl(s: &str) -> bool {
    let re = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
    re.is_match(s)
}

fn check_ecl(s: &str) -> bool {
    matches!(s, "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth")
}

fn check_pid(s: &str) -> bool {
    let re = Regex::new(r"^\d{9}$").unwrap();
    re.is_match(s)
}

fn file_to_vec() -> Vec<String> {
    include_str!("input/day4.txt")
        .split("\n\n")
        .map(|line| line.to_string())
        .collect()
}

pub fn part1() -> usize {
    file_to_vec()
        .iter()
        .filter_map(|passport| Passport::all_fields_present(passport))
        .count()
}

pub fn part2() -> usize {
    file_to_vec()
        .iter()
        .filter_map(|passport| Passport::new(passport.as_str()))
        .count()
}

#[cfg(test)]
mod tests {
    use crate::day4::{part1, part2};

    #[test]
    fn test_part1() {
        assert_eq!(part1(), 196)
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), 114)
    }
}
