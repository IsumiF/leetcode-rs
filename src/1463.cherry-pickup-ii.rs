/*
 * @lc app=leetcode id=1463 lang=rust
 *
 * [1463] Cherry Pickup II
 */

struct Solution {}

// @lc code=start
impl Solution {
    pub fn cherry_pickup(grid: Vec<Vec<i32>>) -> i32 {
        let num_rows = grid.len();
        let num_cols = grid[0].len();
        // [row][col1][col2] -> max value starting from here
        let mut dp: Vec<Vec<Vec<i32>>> = vec![vec![vec![0; num_cols]; num_cols]; num_rows];
        {
            let row = num_rows - 1;
            for c1 in 0..num_cols {
                for c2 in 0..num_cols {
                    dp[row][c1][c2] = if c1 == c2 {
                        grid[row][c1]
                    } else {
                        grid[row][c1] + grid[row][c2]
                    };
                }
            }
        }
        for row in (0..num_rows - 1).rev() {
            for c1 in 0..num_cols {
                for c2 in 0..num_cols {
                    let cur_val = if c1 == c2 {
                        grid[row][c1]
                    } else {
                        grid[row][c1] + grid[row][c2]
                    };
                    let next_val = (c1 as i64 - 1..=c1 as i64 + 1)
                        .flat_map(|next_c1| {
                            (c2 as i64 - 1..=c2 as i64 + 1).map(move |next_c2| (next_c1, next_c2))
                        })
                        .map(|(next_c1, next_c2)| {
                            if next_c1 < 0
                                || next_c1 >= num_cols as i64
                                || next_c2 < 0
                                || next_c2 >= num_cols as i64
                            {
                                -1
                            } else {
                                dp[row + 1][next_c1 as usize][next_c2 as usize]
                            }
                        })
                        .max_by_key(|x: &i32| *x)
                        .unwrap();
                    dp[row][c1][c2] = cur_val + next_val;
                }
            }
        }
        dp[0][0][num_cols - 1]
    }
}

// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(vec![vec![3, 1,1], vec![2, 5, 1], vec![1, 5, 5], vec![2, 1, 1]] => 24)]
    fn test(grid: Vec<Vec<i32>>) -> i32 {
        Solution::cherry_pickup(grid)
    }
}
