/*
 * @lc app=leetcode id=152 lang=rust
 *
 * [152] Maximum Product Subarray
 */

struct Solution {}

// @lc code=start
impl Solution {
    pub fn max_product(nums: Vec<i32>) -> i32 {
        nums.into_iter()
            .fold(None, |acc_option, val| match acc_option {
                None => Some(Acc {
                    min: val,
                    max: val,
                    min_at_bound: val,
                    max_at_bound: val,
                }),
                Some(acc) => {
                    let bound_vals = [acc.min_at_bound * val, acc.max_at_bound * val, val];
                    let new_max_at_bound = *bound_vals.iter().max().unwrap_or(&1);
                    let new_min_at_bound = *bound_vals.iter().min().unwrap_or(&1);
                    Some(Acc {
                        min: std::cmp::min(new_min_at_bound, acc.min),
                        max: std::cmp::max(new_max_at_bound, acc.max),
                        min_at_bound: new_min_at_bound,
                        max_at_bound: new_max_at_bound,
                    })
                }
            })
            .map(|acc| acc.max)
            .unwrap_or(0)
    }
}

struct Acc {
    min: i32,
    max: i32,
    min_at_bound: i32,
    max_at_bound: i32,
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(vec![-2,0,-1] => 0)]
    fn test(nums: Vec<i32>) -> i32 {
        Solution::max_product(nums)
    }
}
