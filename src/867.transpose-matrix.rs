/*
 * @lc app=leetcode id=867 lang=rust
 *
 * [867] Transpose Matrix
 */

struct Solution {}

// @lc code=start
impl Solution {
    pub fn transpose(matrix: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let rows = matrix.len();
        let cols = matrix[0].len();
        let mut result = vec![vec![0i32; rows]; cols];
        for i in 0..rows {
            for j in 0..cols {
                result[j][i] = matrix[i][j];
            }
        }
        result
    }
}
// @lc code=end
