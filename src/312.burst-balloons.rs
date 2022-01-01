/*
 * @lc app=leetcode id=312 lang=rust
 *
 * [312] Burst Balloons
 */

struct Solution {}

// @lc code=start
impl Solution {
    pub fn max_coins(nums: Vec<i32>) -> i32 {
        let nums = std::iter::once(1)
            .chain(nums.into_iter().filter(|&x| x != 0))
            .chain(std::iter::once(1))
            .collect::<Vec<i32>>();
        println!("nums: {:?}", nums);

        let mut dp = vec![vec![0; nums.len()]; nums.len()];
        for k in 2..nums.len() {
            for left in 0..(nums.len() - k) {
                let right = left + k;
                for i in (left + 1)..right {
                    dp[left][right] = std::cmp::max(
                        dp[left][right],
                        nums[left] * nums[i] * nums[right] + dp[left][i] + dp[i][right],
                    );
                }
            }
        }
        dp[0][nums.len() - 1]
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(vec![3,1,5,8] => 167)]
    fn test(nums: Vec<i32>) -> i32 {
        Solution::max_coins(nums)
    }
}
