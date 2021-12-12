/*
 * @lc app=leetcode id=416 lang=rust
 *
 * [416] Partition Equal Subset Sum
 */

struct Solution {}

// @lc code=start
use std::collections::HashMap;

impl Solution {
    pub fn can_partition(nums: Vec<i32>) -> bool {
        Solver::new().solve(nums)
    }
}

struct Solver {
    cache: HashMap<(usize, i32), bool>,
}

impl Solver {
    fn new() -> Self {
        Self {
            cache: HashMap::new(),
        }
    }

    fn solve(mut self, nums: Vec<i32>) -> bool {
        let sum: i32 = nums.iter().sum();
        if sum % 2 == 0 {
            self.solve_rec(&nums, sum / 2)
        } else {
            false
        }
    }

    fn solve_rec(&mut self, nums: &[i32], sum: i32) -> bool {
        let key = (nums.len(), sum);
        match self.cache.get(&key) {
            Some(&cached_value) => cached_value,
            None => {
                let result = match nums.split_last() {
                    None => sum == 0,
                    Some((last, rem)) => {
                        self.solve_rec(rem, sum - last) || self.solve_rec(rem, sum)
                    }
                };
                self.cache.insert(key, result);
                result
            }
        }
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(vec![1,5,11,5] => true)]
    #[test_case(vec![1,2,3,5] => false)]
    fn test(nums: Vec<i32>) -> bool {
        Solution::can_partition(nums)
    }
}
