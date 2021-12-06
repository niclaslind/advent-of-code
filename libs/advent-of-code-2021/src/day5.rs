use std::collections::BTreeMap;

use nom::{
    bytes::complete::tag,
    character::complete::{digit1, newline},
    multi::separated_list1,
    IResult,
};

#[derive(Debug, Ord, PartialOrd, PartialEq, Eq)]
pub struct Point {
    pub x: i16,
    pub y: i16,
}

pub fn point(input: &str) -> IResult<&str, Point> {
    let (input, x) = digit1(input)?;
    let (input, _) = tag(",")(input)?;
    let (input, y) = digit1(input)?;

    Ok((
        input,
        Point {
            x: x.parse().unwrap(),
            y: y.parse().unwrap(),
        },
    ))
}

pub fn points(input: &str) -> IResult<&str, (Point, Point)> {
    let (input, a) = point(input)?;
    let (input, _) = tag(" -> ")(input)?;
    let (input, b) = point(input)?;
    Ok((input, (a, b)))
}

pub fn read_puzzle_input(input: &str) -> IResult<&str, Vec<(Point, Point)>> {
    let (input, lines) = separated_list1(newline, points)(input)?;
    Ok((input, lines))
}

pub fn part1(input: &str) -> usize {
    let (_, lines) = read_puzzle_input(input).unwrap();
    let mut points: BTreeMap<Point, u8> = BTreeMap::new();
    for (a, b) in lines {
        let dx = b.x - a.x;
        let dy = b.y - a.y;

        if dx == 0 {
            let pt = {
                match dy.signum() {
                    1 => a,
                    -1 => b,
                    _ => {
                        panic!("nope");
                    }
                }
            };
            for new_y in pt.y..=(pt.y + dy.abs()) {
                // println!("inserting {} {}", &pt.x, new_y);
                // println!("inserting ({},{})", &pt.x, new_y);

                points
                    .entry(Point { x: pt.x, y: new_y })
                    .and_modify(|v| *v += 1)
                    .or_insert(1);
            }
        } else if dy == 0 {
            let pt = {
                match dx.signum() {
                    1 => a,
                    -1 => b,
                    _ => {
                        unreachable!("YOU SHOULD NOT PASS")
                    }
                }
            };
            for new_x in pt.x..=(pt.x + dx.abs()) {
                points
                    .entry(Point { x: new_x, y: pt.y })
                    .and_modify(|v| *v += 1)
                    .or_insert(1);
            }
        } else {
            continue;
        }
    }
    points.iter().filter(|(_point, &count)| count >= 2).count()
}

pub fn part2(input: &str) -> usize {
    let (_, lines) = read_puzzle_input(input).unwrap();
    let mut points: BTreeMap<Point, u8> = BTreeMap::new();
    for (a, b) in lines {
        let dx = b.x - a.x;
        let dy = b.y - a.y;

        if dx == 0 {
            let pt = {
                match dy.signum() {
                    1 => a,
                    -1 => b,
                    _ => {
                        panic!("nope");
                    }
                }
            };
            for new_y in pt.y..=(pt.y + dy.abs()) {
                points
                    .entry(Point { x: pt.x, y: new_y })
                    .and_modify(|v| *v += 1)
                    .or_insert(1);
            }
        } else if dy == 0 {
            let pt = {
                match dx.signum() {
                    1 => a,
                    -1 => b,
                    _ => {
                        panic!("nope");
                    }
                }
            };
            for new_x in pt.x..=(pt.x + dx.abs()) {
                points
                    .entry(Point { x: new_x, y: pt.y })
                    .and_modify(|v| *v += 1)
                    .or_insert(1);
            }
        } else {
            let sx = dx.signum();
            let sy = dy.signum();
            for i in 0..=dx.abs() {
                let new_point = Point {
                    x: a.x + i * sx,
                    y: a.y + i * sy,
                };
                points.entry(new_point).and_modify(|v| *v += 1).or_insert(1);
            }
        }
    }
    points.iter().filter(|(_point, &count)| count >= 2).count()
}

#[cfg(test)]
mod tests {
    use crate::day5::{part1, part2};

    const INPUT: &str = include_str!("./input/day5.txt");

    #[test]
    fn test_part1() {
        assert_eq!(6548, part1(INPUT));
    }

    #[test]
    fn test_part2() {
        assert_eq!(19663, part2(INPUT));
    }
}
