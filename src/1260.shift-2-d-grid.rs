/*
 * @lc app=leetcode id=1260 lang=rust
 *
 * [1260] Shift 2D Grid
 */

struct Solution {}

// @lc code=start
impl Solution {
    pub fn shift_grid(grid: Vec<Vec<i32>>, k_: i32) -> Vec<Vec<i32>> {
        if grid.is_empty() {
            return grid;
        }
        let k = k_ as usize;
        let width = grid[0].len();
        let height = grid.len();
        let mut result: Vec<Vec<i32>> = vec![vec![0; width]; height];
        for (i, row) in grid.into_iter().enumerate() {
            for (j, v) in row.into_iter().enumerate() {
                let new_j = (j + k) % width;
                let new_i = (i + (j + k) / width) % height;
                result[new_i][new_j] = v;
            }
        }

        result
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(vec![vec![1,2,3],vec![4,5,6],vec![7,8,9]], 1 => vec![vec![9,1,2],vec![3,4,5],vec![6,7,8]])]
    fn test(grid: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        Solution::shift_grid(grid, k)
    }
}
