use anyhow::{Context, Result};

fn main() -> Result<()> {
    let solution_1 = solve_part_one();

    let solution_2 = solve_part_two();

    println!("The solution of part one is {:?}", solution_1);

    println!("The solution of part two is {:?}", solution_2);

    return Ok(());
}

fn solve_part_one() -> Result<usize> {
    let max = include_str!("./day1.input")
        .split("\n\n")
        .map(|x| {
            return x.lines().flat_map(str::parse::<usize>).sum::<usize>();
        })
        .max()
        .context("Failed");

    return max;
}

fn solve_part_two() -> Result<usize> {
    let mut vector: Vec<usize> = include_str!("./day1.input")
        .split("\n\n")
        .map(|x| {
            return x.lines().flat_map(str::parse::<usize>).sum::<usize>();
        })
        .collect();

    vector.sort_by(|a, b| b.cmp(a));

    let max_value = vector.into_iter().take(3).sum();

    return Ok(max_value);
}
