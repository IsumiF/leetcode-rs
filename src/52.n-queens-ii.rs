/*
 * @lc app=leetcode id=52 lang=rust
 *
 * [52] N-Queens II
 */

struct Solution {}

// @lc code=start
use std::iter::FromIterator;

impl Solution {
    pub fn total_n_queens(n: i32) -> i32 {
        Self::solve_n_queens(n).len() as i32
    }

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
