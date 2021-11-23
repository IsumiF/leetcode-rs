/*
 * @lc app=leetcode id=448 lang=rust
 *
 * [448] Find All Numbers Disappeared in an Array
 */

struct Solution {}

// @lc code=start
use std::convert::TryInto;

impl Solution {
    pub fn find_disappeared_numbers(mut nums: Vec<i32>) -> Vec<i32> {
        let n: i32 = nums.len().try_into().unwrap();
        nums.sort();

        let mut result: Vec<i32> = Vec::new();
        let mut expected_num: i32 = 1;
        for num in nums.into_iter() {
            if num > expected_num && expected_num <= n {
                for missing in expected_num..num {
                    result.push(missing);
                }
            }
            expected_num = num + 1;
        }

        if expected_num <= n {
            for missing in expected_num..=n {
                result.push(missing);
            }
        }
        result
    }
}
// @lc code=end

mod tests {
    #[test]
    fn simple() {
        assert_eq!(
            super::Solution::find_disappeared_numbers(vec![1, 3, 3, 4, 4]),
            vec![2, 5]
        );
    }
}
