use anyhow::Result;
use std::str::FromStr;

struct Zone {
    start: u32,
    end: u32,
}

struct Pair {
    left: Zone,
    right: Zone,
}

impl FromStr for Zone {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        return match s.split_once("-") {
            Some((start, end)) => Ok(Self {
                start: start.parse()?,
                end: end.parse()?,
            }),
            _ => panic!("Error deserializing Zone: {s}"),
        };
    }
}

impl Zone {
    fn is_contained(&self, other: &Self) -> bool {
        return other.start <= self.start && other.end >= self.end;
    }
}

impl FromStr for Pair {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        return match s.split_once(",") {
            Some((left, right)) => Ok(Self {
                left: left.parse()?,
                right: right.parse()?,
            }),
            _ => panic!("Error parsing lines"),
        };
    }
}

impl Pair {
    fn contained(&self) -> bool {
        return self.left.is_contained(&self.right) || self.right.is_contained(&self.left);
    }

    fn have_overlap(&self) -> bool {
        return self.left.start <= self.right.start && self.left.end >= self.right.start
            || (self.right.start <= self.left.start && self.right.end >= self.left.start);
    }
}

fn main() -> Result<()> {
    let solution_1 = solve_part_one();

    println!("The solution of part one is {:?}", solution_1);

    let solution_2 = solve_part_two();

    println!("The solution of part two is {:?}", solution_2);

    return Ok(());
}

fn solve_part_one() -> Result<usize> {
    let included_zones = include_str!("./day4.input")
        .lines()
        .map(|line| return line.parse::<Pair>().expect("Error parsing zones pairs"))
        .filter(Pair::contained)
        .count();

    return Ok(included_zones);
}

fn solve_part_two() -> Result<usize> {
    let included_zones = include_str!("./day4.input")
        .lines()
        .map(|line| return line.parse::<Pair>().expect("Error parsing zones pairs"))
        .filter(Pair::have_overlap)
        .count();

    return Ok(included_zones);
}

