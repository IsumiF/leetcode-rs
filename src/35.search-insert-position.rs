/*
 * @lc app=leetcode id=35 lang=rust
 *
 * [35] Search Insert Position
 */

struct Solution {}

// @lc code=start
use std::convert::TryInto;

impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        match nums.binary_search(&target) {
            Ok(idx) => idx,
            Err(idx) => idx,
        }
        .try_into()
        .unwrap()
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(vec![1, 3, 5, 6], 5 => 2)]
    #[test_case(vec![1, 3, 5, 6], 2 => 1)]
    #[test_case(vec![1, 3, 5, 6], 7 => 4)]
    #[test_case(vec![1, 3, 5, 6], 0 => 0)]
    #[test_case(vec![1], 0 => 0)]
    fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        Solution::search_insert(nums, target)
    }
}
