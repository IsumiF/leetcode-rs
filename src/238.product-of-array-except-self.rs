/*
 * @lc app=leetcode id=238 lang=rust
 *
 * [238] Product of Array Except Self
 */

struct Solution {}

// @lc code=start
impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let mut result: Vec<i32> = vec![1; nums.len()];
        let mut prefix: i32 = 1;
        let mut suffix: i32 = 1;
        for i in 0..nums.len() {
            result[i] *= prefix;
            result[nums.len() - 1 - i] *= suffix;

            prefix *= nums[i];
            suffix *= nums[nums.len() - 1 - i];
        }
        result
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(vec![1, 2, 3, 4] => vec![24, 12, 8, 6])]
    #[test_case(vec![-1,1,0,-3,3] => vec![0, 0, 9, 0, 0])]
    fn test(nums: Vec<i32>) -> Vec<i32> {
        Solution::product_except_self(nums)
    }
}
