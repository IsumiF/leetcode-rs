/*
 * @lc app=leetcode id=51 lang=rust
 *
 * [51] N-Queens
 */

struct Solution {}

// @lc code=start
use std::iter::FromIterator;

impl Solution {
    pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
        let mut solver = Solver::new(n as usize);
        solver.solve();
        solver.get_solutions()
    }
}

struct Solver {
    board_width: usize,
    placed_queens: Vec<usize>,
    known_solutions: Vec<Vec<usize>>,
}

impl Solver {
    fn new(n: usize) -> Self {
        let mut placed_queens = Vec::new();
        placed_queens.reserve_exact(n);
        Self {
            board_width: n,
            placed_queens,
            known_solutions: Vec::new(),
        }
    }

    fn solve(&mut self) {
        if self.placed_queens.len() == self.board_width {
            self.known_solutions.push(self.placed_queens.clone());
        }

        for i in 0..self.board_width {
            let mut attacks = false;
            for (r, &c) in self.placed_queens.iter().enumerate() {
                if Self::attacks(self.placed_queens.len(), i, r, c) {
                    attacks = true;
                    break;
                }
            }
            if !attacks {
                self.placed_queens.push(i);
                self.solve();
                self.placed_queens.pop();
            }
        }
    }

    fn get_solutions(self) -> Vec<Vec<String>> {
        let board_width = self.board_width;
        self.known_solutions
            .into_iter()
            .map(|solution| {
                solution
                    .into_iter()
                    .map(|col| {
                        String::from_iter(
                            std::iter::repeat('.')
                                .take(col)
                                .chain(std::iter::once('Q'))
                                .chain(std::iter::repeat('.').take(board_width - col - 1)),
                        )
                    })
                    .collect()
            })
            .collect()
    }

    fn attacks(xr: usize, xc: usize, yr: usize, yc: usize) -> bool {
        xr == yr || xc == yc || (xr as i64 - yr as i64).abs() == (xc as i64 - yc as i64).abs()
    }
}

// @lc code=end

#[cfg(test)]
mod tests {
    use super::Solution;
    use test_case::test_case;

    #[test_case(1 => vec![vec!["Q"]])]
    #[test_case(4 => vec![vec![".Q..","...Q","Q...","..Q."],vec!["..Q.","Q...","...Q",".Q.."]])]
    fn test(n: i32) -> Vec<Vec<String>> {
        Solution::solve_n_queens(n)
    }
}
