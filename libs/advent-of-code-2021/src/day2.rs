use nom::{branch::alt, bytes::complete::tag, character::complete::i32, IResult};

#[derive(Default, Debug)]
struct Submarine {
    x: i32,
    y: i32,
    aim: i32,
}

impl Submarine {
    fn with_aim() -> Self {
        Self { x: 0, y: 0, aim: 0 }
    }

    fn final_destination(&self) -> i32 {
        self.x * self.y
    }
}

enum Direction {
    Forward(i32),
    Up(i32),
    Down(i32),
}

fn parse_direction(input: &str) -> IResult<&str, Direction> {
    let (input, direction) = alt((tag("forward"), tag("up"), tag("down")))(input)?;

    let (input, _) = tag(" ")(input)?;
    let (input, value) = i32(input)?;

    let result = match direction {
        "forward" => Direction::Forward(value),
        "up" => Direction::Up(value),
        "down" => Direction::Down(value),
        _ => unreachable!(),
    };

    Ok((input, result))
}

pub fn part1() -> i32 {
    let submarine = include_str!("input/day2.txt").lines().fold(
        Submarine::default(),
        |mut submarine, directions| {
            let (_, direction) = parse_direction(directions).unwrap();

            match direction {
                Direction::Forward(value) => submarine.x += value,
                Direction::Down(value) => submarine.y += value,
                Direction::Up(value) => submarine.y -= value,
            };
            submarine
        },
    );

    submarine.final_destination()
}

pub fn part2() -> i32 {
    let submarine = include_str!("input/day2.txt").lines().fold(
        Submarine::with_aim(),
        |mut submarine, directions| {
            let (_, direction) = parse_direction(directions).unwrap();

            match direction {
                Direction::Forward(value) => {
                    submarine.x += value;
                    submarine.y += submarine.aim * value;
                }
                Direction::Down(value) => submarine.aim += value,
                Direction::Up(value) => submarine.aim -= value,
            };
            submarine
        },
    );

    submarine.final_destination()
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
