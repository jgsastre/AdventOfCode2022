use anyhow::Result;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, Clone)]
enum Play {
    Rock,
    Paper,
    Scissors,
}

impl FromStr for Play {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        return match s {
            "A" | "X" => Ok(Play::Rock),
            "B" | "Y" => Ok(Play::Paper),
            "C" | "Z" => Ok(Play::Scissors),
            _ => unreachable!("invalid input"),
        };
    }
}

impl Play {
    fn score(&self) -> u8 {
        return match self {
            Play::Rock => 1,
            Play::Paper => 2,
            Play::Scissors => 3,
        };
    }

    fn win(&self) -> Play {
        return match self {
            Play::Rock => Play::Paper,
            Play::Paper => Play::Scissors,
            Play::Scissors => Play::Rock
        }
    }

    fn lose(&self) -> Play {
        return match self {
            Play::Rock => Play::Scissors,
            Play::Paper => Play::Rock,
            Play::Scissors => Play::Paper
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Hand {
    left: Play,
    right: Play,
}

impl FromStr for Hand {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        return match s.split_once(" ") {
            Some((l, r)) => Ok(Self {
                left: l.parse()?,
                right: r.parse()?,
            }),
            _ => Err(anyhow::anyhow!("invalid input")),
        };
    }
}

impl Hand {
    fn wins(&self) -> u8 {
        return match self {
            Hand {
                left: Play::Rock,
                right: Play::Paper,
            }
            | Hand {
                left: Play::Paper,
                right: Play::Scissors,
            }
            | Hand {
                left: Play::Scissors,
                right: Play::Rock,
            } => 6,
            Hand {
                left: Play::Paper,
                right: Play::Rock,
            }
            | Hand {
                left: Play::Scissors,
                right: Play::Paper,
            }
            | Hand {
                left: Play::Rock,
                right: Play::Scissors,
            } => 0,
            _ => 3,
        };
    }
}

#[derive(Debug, PartialEq, Eq)]
enum HandResult {
    Lose,
    Draw,
    Win,
}

impl FromStr for HandResult {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        return match s {
            "X" => Ok(HandResult::Lose),
            "Y" => Ok(HandResult::Draw),
            "Z" => Ok(HandResult::Win),
            _ => unreachable!("invalid input"),
        };
    }
}

#[derive(Debug, PartialEq, Eq)]
struct HandSecondPart {
    left: Play,
    desired: HandResult,
}

impl FromStr for HandSecondPart {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        return match s.split_once(" ") {
            Some((l, r)) => Ok(Self {
                left: l.parse()?,
                desired: r.parse()?,
            }),
            _ => Err(anyhow::anyhow!("invalid input")),
        };
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
    let score = include_str!("./day2.input")
        .lines()
        .flat_map(|line| {
            let hand = line.parse::<Hand>()?;
            return Ok::<usize, anyhow::Error>((hand.wins() + hand.right.score()) as usize);
        }).sum::<usize>();

    return Ok(score);
}

fn solve_part_two() -> Result<usize> {
    let score = include_str!("./day2.input")
        .lines()
        .flat_map(|line| {
            let hand = line.parse::<HandSecondPart>()?;
            let second_play = match hand.desired {
                HandResult::Win => hand.left.win(),
                HandResult::Lose => hand.left.lose(),
                _ => hand.left.clone()
            };
            let final_hand = Hand{left: hand.left, right: second_play};
            return Ok::<usize, anyhow::Error>((final_hand.wins() + final_hand.right.score()) as usize);
        }).sum::<usize>();

    return Ok(score);
}

#[cfg(test)]
mod tests {
    use crate::{Hand, Play};

    #[test]
    fn parse_line() {
        let hand = "A Y".parse::<Hand>();

        assert!(matches!(
            hand,
            Ok(Hand {
                left: Play::Rock,
                right: Play::Paper
            })
        ));
    }

    #[test]
    fn play_score() {
        assert_eq!(Play::Rock.score(), 1);
    }

    #[test]
    fn hand_contest() {
        let hand = Hand{ left: Play::Rock, right: Play::Paper};
        assert_eq!(hand.wins(), 6);
    }
}
