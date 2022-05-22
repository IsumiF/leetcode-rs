/*
 * @lc app=leetcode id=647 lang=rust
 *
 * [647] Palindromic Substrings
 */

struct Solution {}

// @lc code=start
impl Solution {
    pub fn count_substrings(s: String) -> i32 {
        let chars: Vec<char> = s.chars().collect();
        let mut dp: Vec<Vec<bool>> = vec![vec![false; chars.len()]; chars.len()];
        for i in 0..dp.len() {
            dp[i][i] = true;
        }
        for i in 0..dp.len() - 1 {
            dp[i][i + 1] = chars[i] == chars[i + 1];
        }
        for step in 2..dp.len() {
            for i in 0..dp.len() - step {
                let j = i + step;
                dp[i][j] = dp[i + 1][j - 1] && chars[i] == chars[j];
            }
        }
        dp.into_iter().flatten().filter(|&b| b).count() as i32
    }
}
// @lc code=end
