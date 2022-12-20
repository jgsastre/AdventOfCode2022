use std::{collections::HashMap, str::FromStr};

use anyhow::{anyhow, Result};
use itertools::Itertools;
//use itertools::Itertools;

#[derive(Debug)]
struct Crane {
    stack: HashMap<u8, Vec<char>>,
}

impl Crane {
    fn move_one_by_one(&mut self, movement: &Move) -> &Self {
        let _removed_ements: Vec<_> = (0..movement.quantity)
            .map(|_| {
                let item = self
                    .stack
                    .get_mut(&movement.origin)
                    .expect("Origin must exists")
                    .pop()
                    .unwrap();
                self.stack
                    .get_mut(&movement.destiny)
                    .expect("Destiny must exists")
                    .push(item);
            })
            .collect();

        self
    }

    fn move_in_bulk(&mut self, movement: &Move) -> &Self {
        let removed_ements = (0..movement.quantity)
            .map(|_| {
                self.stack
                    .get_mut(&movement.origin)
                    .expect("Origin must exists")
                    .pop()
                    .unwrap()
            })
            .collect::<Vec<_>>();

        removed_ements
            .into_iter()
            .rev()
            .for_each(|x| self.stack.get_mut(&movement.destiny).unwrap().push(x));

        self
    }
}

impl FromStr for Crane {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut stack = HashMap::<u8, Vec<char>>::new();

        for line in s.lines().rev().skip(1) {
            let mut idx: u8 = 1;
            let mut i = 0;
            while i < line.len() {
                let token = &line[i..(i + 3)];
                if !token.trim().is_empty() {
                    let value = token[1..].chars().next().expect("Empty token!");
                    stack
                        .entry(idx)
                        .and_modify(|elements| elements.push(value))
                        .or_insert(vec![value]);
                }
                i += 4;
                idx += 1;
            }
        }

        return Ok(Crane { stack });
    }
}

#[derive(Debug)]
struct Move {
    quantity: usize,
    origin: u8,
    destiny: u8,
}

impl FromStr for Move {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        match s.split(" ").collect::<Vec<_>>()[..] {
            [_, q, _, o, _, d] => Ok(Self {
                quantity: q.parse()?,
                origin: o.parse()?,
                destiny: d.parse()?,
            }),
            _ => Err(anyhow!("Imposible parse {s}")),
        }
    }
}

fn main() -> Result<()> {

    let solution_1 = solve_part_one();

    println!("The solution of part one is {:?}", solution_1);

    let solution_2 = solve_part_two();

    println!("The solution of part two is {:?}", solution_2);

    return Ok(());
}

fn solve_part_one() -> Result<String> {
    let (crane, movements) = include_str!("./day5.input").split_once("\n\n").unwrap();
    let mut initial_crane: Crane = crane.parse()?;
    movements
        .lines()
        .map(|line| line.parse::<Move>().unwrap())
        .for_each(|ref m| {
            initial_crane.move_one_by_one(m);
        });

    let mut result = String::new();
    initial_crane
        .stack
        .into_iter()
        .sorted_by(|a, b| Ord::cmp(&a.0, &b.0))
        .for_each(|(_, v)| {
            if let Some(&c) = v.last() {
                result.push(c);
            }
        });

    return Ok(result);
}

fn solve_part_two() -> Result<String> {
    let (crane, movements) = include_str!("./day5.input").split_once("\n\n").unwrap();
    let mut initial_crane: Crane = crane.parse()?;
    movements
        .lines()
        .map(|line| line.parse::<Move>().unwrap())
        .for_each(|ref m| {
            initial_crane.move_in_bulk(m);
        });

    let mut result = String::new();
    initial_crane
        .stack
        .into_iter()
        .sorted_by(|a, b| Ord::cmp(&a.0, &b.0))
        .for_each(|(_, v)| {
            if let Some(&c) = v.last() {
                result.push(c);
            }
        });

    return Ok(result);
}

#[cfg(test)]
#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_parse_header() {
        let header = include_str!("./day5.test").split_once("\n\n").unwrap().0;
        let crates = header.parse::<Crane>().unwrap();
        println!("This is the result: {:?}", crates);
    }
}
