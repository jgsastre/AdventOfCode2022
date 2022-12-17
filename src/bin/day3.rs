use itertools::Itertools;
use std::collections::HashSet;

use anyhow::{anyhow, Context, Result};

#[derive(Clone)]
struct Rucksack<'a> {
    left: &'a str,
    right: &'a str,
}

impl<'a> Rucksack<'a> {
    fn new(line: &'a str) -> Self {
        let size = line.len();
        let limit = size / 2;
        Self {
            left: &line[..limit],
            right: &line[limit..],
        }
    }

    fn get_repeated_item(&self) -> char {
        let left_set: HashSet<_> = self.left.chars().collect();
        let rigth_set: HashSet<_> = self.right.chars().collect();
        *left_set
            .intersection(&rigth_set)
            .next()
            .expect("There should be an intersecton")
    }

    fn get_items(&self) -> HashSet<char> {
        let left_set: HashSet<_> = self.left.chars().collect();
        let righ_set: HashSet<_> = self.right.chars().collect();
        left_set.union(&righ_set).copied().collect()
    }
}

fn get_priority(c: &char) -> Result<usize> {
    let d = *c as u32;
    if d <= 'z' as u32 && d >= 'a' as u32 {
        Ok((d - 'a' as u32 + 1) as usize)
    } else {
        Ok((d - 'A' as u32 + 27) as usize)
    }
}

fn intersection(r1: &Rucksack, r2: &Rucksack, r3: &Rucksack) -> Result<char> {
    let intersection: HashSet<_> = r1
        .get_items()
        .intersection(&r2.get_items())
        .copied()
        .collect();
    let common_element: Vec<_> = r3
        .get_items()
        .intersection(&intersection)
        .copied()
        .collect();
    if common_element.len() != 1 {
        Err(anyhow!("Too many elements in common"))
    } else {
        common_element
            .first()
            .context("Failed getting common element")
            .copied()
    }
}

fn main() -> Result<()> {
    let solution_1 = solve_part_one();

    let solution_2 = solve_part_two();

    println!("The solution of part one is {:?}", solution_1);

    println!("The solution of part two is {:?}", solution_2);

    return Ok(());
}

fn solve_part_one() -> Result<usize> {
    let total_score = include_str!("./day3.input")
        .lines()
        .flat_map(|line| {
            let rep_item = Rucksack::new(line).get_repeated_item();
            get_priority(&rep_item)
        })
        .sum();

    return Ok(total_score);
}

fn solve_part_two() -> Result<usize> {
    let total_score = include_str!("./day3.input")
        .lines()
        .map(Rucksack::new)
        .tuples()
        .flat_map(|x| {
            let (r1, r2, r3) = &x;
            let common_element = intersection(r1, r2, r3)?;
            println!("Common element {common_element}");
            let priority = get_priority(&common_element);
            println!("priority: {priority:?}");
            priority
        })
        .sum();

    return Ok(total_score);
}
