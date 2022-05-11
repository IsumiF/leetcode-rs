/*
 * @lc app=leetcode id=1641 lang=rust
 *
 * [1641] Count Sorted Vowel Strings
 */

struct Solution {}

// @lc code=start
impl Solution {
    pub fn count_vowel_strings(n: i32) -> i32 {
        let mut dp: Vec<Vec<i32>> = vec![vec![0; 5]; n as usize + 1];
        dp[1] = vec![1, 1, 1, 1, 1];
        for i in 2..dp.len() {
            dp[i][0] = dp[i-1][0] + dp[i-1][1] + dp[i-1][2] + dp[i-1][3] + dp[i-1][4];
            dp[i][1] = dp[i-1][1] + dp[i-1][2] + dp[i-1][3] + dp[i-1][4];
            dp[i][2] = dp[i-1][2] + dp[i-1][3] + dp[i-1][4];
            dp[i][3] = dp[i-1][3] + dp[i-1][4];
            dp[i][4] = dp[i-1][4];
        }
        dp.last().unwrap().into_iter().sum()
    }
}

// @lc code=end

#[cfg(test)]
mod tests {
    use super::Solution;
    use test_case::test_case;

    #[test_case(1 => 5)]
    #[test_case(2 => 15)]
    #[test_case(33 => 66045)]
    fn test(n: i32) -> i32 {
        Solution::count_vowel_strings(n)
    }
}