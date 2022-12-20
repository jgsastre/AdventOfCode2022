use anyhow::Result;
use std::collections::HashSet;

fn start_of_packer_marker(s: &str, d: usize) -> Option<usize> {

    if s.len() < d {
        return None;
    }

    for i in d..s.len() { if s[(i-d)..i].chars().collect::<HashSet<_>>().len() == d {
            return Some(i);
        }
    }

    None
}

fn main() -> Result<()> {

    let solution_1 = solve_part_one();

    println!("The solution of part one is {:?}", solution_1);

    let solution_2 = solve_part_two();

    println!("The solution of part two is {:?}", solution_2);

    return Ok(());
}

fn solve_part_one() -> Option<usize> {

    let input = include_str!("./day6.input");
    return start_of_packer_marker(input, 4);

}

fn solve_part_two() -> Option<usize> {

    let input = include_str!("./day6.input");
    return start_of_packer_marker(input, 14);

}

#[cfg(test)]
mod tests {
    use crate::start_of_packer_marker;


    #[test]
    fn test_case_examples_part_one() {
        let result = start_of_packer_marker("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 4);
        assert_eq!(Some(7), result);
        let result = start_of_packer_marker("bvwbjplbgvbhsrlpgdmjqwftvncz", 4);
        assert_eq!(Some(5), result);
        let result = start_of_packer_marker("nppdvjthqldpwncqszvftbrmjlhg", 4);
        assert_eq!(Some(6), result);
        let result = start_of_packer_marker("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 4);
        assert_eq!(Some(10), result);
        let result = start_of_packer_marker("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 4);
        assert_eq!(Some(11), result);
    }

    #[test]
    fn test_case_examples_part_two() {
        let result = start_of_packer_marker("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 14);
        assert_eq!(Some(19), result);
        let result = start_of_packer_marker("bvwbjplbgvbhsrlpgdmjqwftvncz", 14);
        assert_eq!(Some(23), result);
        let result = start_of_packer_marker("nppdvjthqldpwncqszvftbrmjlhg", 14);
        assert_eq!(Some(23), result);
        let result = start_of_packer_marker("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 14);
        assert_eq!(Some(29), result);
        let result = start_of_packer_marker("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 14);
        assert_eq!(Some(26), result);
    }

}
