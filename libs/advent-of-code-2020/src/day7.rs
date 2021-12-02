use std::collections::HashMap;

use regex::Regex;

#[derive(Debug)]
struct Bag {
    color: String,
    contains: Vec<(usize, String)>,
}

impl Bag {
    pub fn new(color: &str) -> Self {
        Self {
            color: color.to_string(),
            contains: Vec::default(),
        }
    }

    pub fn can_contains(&mut self, n: usize, bag: &str) {
        self.contains.push((n, bag.to_string()))
    }

    pub fn can_ultimately_hold(&self, color: &str, set: &HashMap<String, Bag>) -> bool {
        if self.contains.is_empty() {
            return false;
        }

        self.contains.iter().any(|(_, bag)| {
            let bag = bag.as_str();
            bag == color || set.get(bag).unwrap().can_ultimately_hold(color, set)
        })
    }
}

fn count_bags(bag: &Bag, set: &HashMap<String, Bag>) -> usize {
    if bag.contains.is_empty() {
        return 0;
    }

    bag.contains.iter().fold(0, |bags, (n, b)| {
        let inner_count = count_bags(set.get(b.as_str()).unwrap(), set);
        bags + n * (1 + inner_count)
    })
}

fn read_data() -> HashMap<String, Bag> {
    let factory = BagFactory::default();
    let mut set = HashMap::new();
    include_str!("input/day7.txt")
        .lines()
        .filter_map(|line| factory.from_str(line))
        .for_each(|bag| {
            set.insert(bag.color.clone(), bag);
        });
    set
}

struct BagFactory {
    re: Regex,
    re2: Regex,
}

impl Default for BagFactory {
    fn default() -> Self {
        Self {
            re: Regex::new(r"^(.*) bags contain (.*)\.$").unwrap(),
            re2: Regex::new(r"^\s*(\d*) (.*) bags?$").unwrap(),
        }
    }
}

impl BagFactory {
    pub fn from_str<T: AsRef<str>>(&self, s: T) -> Option<Bag> {
        let caps = self.re.captures(s.as_ref())?;
        let color = caps.get(1)?.as_str();
        let mut bag = Bag::new(color);
        let contains = caps.get(2)?.as_str();
        if contains == "no other bags" {
            return Some(bag);
        }

        for s in contains.split(',') {
            let cap = self.re2.captures(s)?;
            let n = cap.get(1)?.as_str().parse::<usize>().unwrap();
            let color = cap.get(2)?.as_str();
            bag.can_contains(n, color);
        }

        Some(bag)
    }
}

pub fn part1() -> usize {
    let bags = read_data();
    bags.values()
        .filter(|bag| bag.can_ultimately_hold("shiny gold", &bags))
        .count()
}

pub fn part2() -> usize {
    let bags = read_data();
    count_bags(bags.get("shiny gold").unwrap(), &bags)
}

#[cfg(test)]
mod tests {
    use crate::day7::part2;

    use super::part1;

    #[test]
    fn test_part1() {
        assert_eq!(274, part1())
    }

    #[test]
    fn test_part2() {
        assert_eq!(158730, part2());
    }
}
