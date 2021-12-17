/*
 * @lc app=leetcode id=221 lang=rust
 *
 * vec![221] Maximal Square
 */

struct Solution {}

// @lc code=start
impl Solution {
    pub fn maximal_square(matrix: Vec<Vec<char>>) -> i32 {
        match matrix.split_first() {
            Some((first_row, _)) => {
                let mut mat: Vec<Vec<usize>> = vec![vec![0; first_row.len()]; matrix.len()];
                for r in 0..mat.len() {
                    for c in 0..first_row.len() {
                        if matrix[r][c] == '1' {
                            if r == 0 || c == 0 {
                                mat[r][c] = 1;
                            } else {
                                let new_width = *[mat[r - 1][c], mat[r - 1][c - 1], mat[r][c - 1]]
                                    .iter()
                                    .min()
                                    .unwrap_or(&1);
                                mat[r][c] = new_width + 1;
                            }
                        }
                    }
                }
                let width = mat.into_iter()
                    .map(|row| row.into_iter().max().unwrap_or(0))
                    .max()
                    .unwrap_or(0) as i32;
                width * width
            }
            None => 0,
        }
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(vec![vec!['1','0','1','0','0'],vec!['1','0','1','1','1'],vec!['1','1','1','1','1']] => 4)]
    fn test(matrix: Vec<Vec<char>>) -> i32 {
        Solution::maximal_square(matrix)
    }
}
