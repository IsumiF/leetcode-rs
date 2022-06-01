/*
 * @lc app=leetcode id=1480 lang=rust
 *
 * [1480] Running Sum of 1d Array
 */

struct Solution {}

// @lc code=start
impl Solution {
    pub fn running_sum(nums: Vec<i32>) -> Vec<i32> {
        nums.into_iter().fold(vec![], |mut acc, x| {
            let sum = *acc.last().unwrap_or(&0) + x;
            acc.push(sum);
            acc
        })
    }
}
// @lc code=end
