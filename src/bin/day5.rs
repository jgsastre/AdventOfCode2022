use std::{
    collections::HashMap,
    str::{FromStr, Lines},
};

use anyhow::Result;
//use itertools::Itertools;

#[derive(Debug)]
struct Crane {
    stack: HashMap<u8, Vec<char>>,
}

impl Crane {
    fn apply_move(&mut self, movement: &Move) -> &Self {
        let removed_ements = (0..movement.quantity)
            .map(|_| {
                self.stack
                    .get_mut(&movement.origin)
                    .expect("Origin must exists")
                    .pop()
                    .unwrap()
            })
            .rev()
            .collect::<Vec<_>>();
        removed_ements
            .into_iter()
            .for_each(|x| self.stack.get_mut(&movement.destiny).unwrap().push(x));

        self
    }
}

impl FromStr for Crane {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut stack = HashMap::<u8, Vec<char>>::new();

        for line in s.lines().rev().skip(1) {
            println!("Parsing line: {line}");
            let mut idx: u8 = 1;
            let mut i = 0;
            while i < line.len() {
                let token = &line[i..(i + 3)];
                println!("Parsing \"{token}\"");
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

struct Move {
    quantity: usize,
    origin: u8,
    destiny: u8,
}

fn main() -> Result<()> {
    let solution_1 = solve_part_one();

    println!("The solution of part one is {:?}", solution_1);

    return Ok(());
}

fn solve_part_one() -> Result<usize> {
    let _it = include_str!("./day5.test").lines();

    return Ok(5);
}

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
