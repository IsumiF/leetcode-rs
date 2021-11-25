/*
 * @lc app=leetcode id=540 lang=rust
 *
 * [540] Single Element in a Sorted Array
 */

struct Solution {}

// @lc code=start
impl Solution {
    pub fn single_non_duplicate(nums: Vec<i32>) -> i32 {
        nums.iter().fold(0, |acc, num| acc ^ num)
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    #[test]
    fn simple() {
        assert_eq!(
            super::Solution::single_non_duplicate(vec![1, 1, 2, 3, 3, 4, 4, 8, 8]),
            2
        );
        assert_eq!(
            super::Solution::single_non_duplicate(vec![3, 3, 7, 7, 10, 11, 11]),
            10
        );
    }
}
