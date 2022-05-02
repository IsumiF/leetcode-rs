/*
 * @lc app=leetcode id=905 lang=rust
 */

struct Solution {}

// @lc code=start
impl Solution {
    pub fn sort_array_by_parity(mut nums: Vec<i32>) -> Vec<i32> {
        nums.sort_by_key(|n| n % 2 != 0); 
        nums
    }
}
// @lc code=end
