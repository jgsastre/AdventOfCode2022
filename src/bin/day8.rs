use anyhow::Result;

#[derive(Debug)]
struct VisibleReport(Vec<Vec<bool>>);

impl VisibleReport {
    fn new(rows: usize, cols: usize) -> Self {
        let mut report = vec![vec![false; cols]; rows];
        report[0] = vec![true; cols];
        report[rows - 1] = vec![true; cols];
        for line in report.iter_mut() {
            line[0] = true;
            line[cols - 1] = true;
        }
        VisibleReport(report)
    }

    fn visible_count(&self) -> usize {

        self.0.iter().map(|line| line.iter().filter(|x| **x).count()).sum()
    }
}

fn main() -> Result<()> {
    let solution_1 = solve_part_one();

    println!("The solution of part one is {:?}", solution_1);

    /* let solution_2 = solve_part_two();

    println!("The solution of part two is {:?}", solution_2); */

    Ok(())
}

fn solve_part_one() -> Result<usize> {
    let grid = include_str!("./day8.input")
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
/*  30373
    25512
    65332
    33549
    35390 */

    let mut report = VisibleReport::new(grid.len(), grid[0].len());
    for i in (0..(grid.len() - 1)).skip(1) {
        let mut iter = grid[i].iter().enumerate();
        let mut max_left = iter.next().unwrap();
        while let Some(left) = iter.next() {
            if *max_left.1 == 9_u8 {
                break;
            }

            if *left.1 > *max_left.1 {
                max_left = left;
                report.0[i][left.0] = true;
            }
        }
        let mut iter = grid[i].iter().enumerate().rev();
        let mut max_right = iter.next().unwrap();
        while let Some(right) = iter.next() {
            if *max_right.1 == 9_u8 || right.0 < max_left.0 {
                break;
            }

            if *right.1 > *max_right.1 {
                max_right = right;
                report.0[i][right.0] = true;
            }
        }
    }

    for col in (0..(grid[0].len() - 1)).skip(1) {
        let mut iter = 0..grid.len();
        let mut max_left = (0_usize, grid[iter.next().unwrap()][col]);
        while let Some(row) = iter.next() {
            if max_left.1 == 9_u8 {
                break;
            }

            let left = grid[row][col];
            if left > max_left.1 {
                max_left = (row, left);
                report.0[row][col] = true;
            }
        }
        let mut iter = (0..grid.len()).rev();
        let mut max_right = grid[iter.next().unwrap()][col];
        while let Some(row) = iter.next() {
            if max_right == 9_u8 || row == max_left.0 {
                break;
            }

            let right = grid[row][col];
            if right > max_right {
                max_right = right;
                report.0[row][col] = true;
            }
        }
    }

    Ok(report.visible_count())
}

#[cfg(test)]
mod test {
    use crate::VisibleReport;

    #[test]
    fn test_visible_report_new() {
        let report = VisibleReport::new(3, 3);
        println!("This is the visible report: {report:?}");
    }
}
