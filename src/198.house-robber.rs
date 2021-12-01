/*
 * @lc app=leetcode id=198 lang=rust
 *
 * [198] House Robber
 */

struct Solution {}

// @lc code=start
impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        let mut result = vec![0; nums.len()];
        if nums.len() >= 1 {
            result[0] = nums[0];
        }
        if nums.len() >= 2 {
            result[1] = std::cmp::max(nums[0], nums[1]);
        }
        if nums.len() >= 3 {
            for i in 2..nums.len() {
                result[i] = std::cmp::max(result[i - 1], result[i - 2] + nums[i]);
            }
        }

        *result.last().unwrap_or(&0)
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(vec![1, 2, 3, 1] => 4)]
    #[test_case(vec![2, 7, 9, 3, 1] => 12)]
    fn test(nums: Vec<i32>) -> i32 {
        Solution::rob(nums)
    }
}
