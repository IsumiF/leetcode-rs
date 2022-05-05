/*
 * @lc app=leetcode id=1679 lang=rust
 *
 * [1679] Max Number of K-Sum Pairs
 */

struct Solution {}

// @lc code=start
use std::collections::HashMap;

impl Solution {
    pub fn max_operations(nums: Vec<i32>, k: i32) -> i32 {
        let mut nums_count: HashMap<i32, usize> = HashMap::new();
        nums.into_iter().for_each(|n| {
            let count = nums_count.remove(&n).unwrap_or(0) + 1;
            nums_count.insert(n, count);
        });
        let mut operations: usize = 0;
        while !nums_count.is_empty() {
            let mut found: bool = false;
            for (&num, &count) in nums_count.iter() {
                let target = k - num;
                if target != num && nums_count.contains_key(&target) {
                    if count >= 2 {
                        nums_count.insert(num, count - 1);
                    } else {
                        nums_count.remove(&num);
                    }
                    let target_count = nums_count.remove(&target).unwrap_or(1);
                    if target_count >= 2 {
                        nums_count.insert(target, target_count - 1);
                    }
                    found = true;
                    break;
                }
                if target == num && count >= 2 {
                    if count >= 3 {
                        nums_count.insert(num, count - 2);
                    } else {
                        nums_count.remove(&num);
                    }
                    found = true;
                    break;
                }
                nums_count.remove(&num);
                found = false;
                break;
            }
            if found {
                operations += 1;
            }
        }
        operations as i32
    }
}
// @lc code=end

#[cfg(test)]
mod tests {

    use super::Solution;
    use test_case::test_case;

    #[test_case(vec![4,4,1,3,1,3,2,2,5,5,1,5,2,1,2,3,5,4], 2 => 2)]
    fn test(nums: Vec<i32>, k: i32) -> i32 {
        Solution::max_operations(nums, k)
    }
}
