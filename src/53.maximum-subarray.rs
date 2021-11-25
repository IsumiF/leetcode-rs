/*
 * @lc app=leetcode id=53 lang=rust
 *
 * [53] Maximum Subarray
 */

struct Solution {}

// @lc code=start
impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        nums.into_iter()
            .fold((i32::MIN, i32::MIN), |(max, max_at_edge), next| {
                let new_max_at_edge = if max_at_edge <= 0 {
                    next
                } else {
                    max_at_edge + next
                };
                let new_max = std::cmp::max(max, new_max_at_edge);

                (new_max, new_max_at_edge)
            })
            .0
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(vec![-2, 1, -3, 4, -1, 2, 1, -5, 4] => 6)]
    #[test_case(vec![1] => 1)]
    #[test_case(vec![5, 4, -1, 7, 8] => 23)]
    fn simple(nums: Vec<i32>) -> i32 {
        Solution::max_sub_array(nums)
    }
}
