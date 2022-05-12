/*
 * @lc app=leetcode id=47 lang=rust
 *
 * [47] Permutations II
 */

struct Solution {}

// @lc code=start
use std::collections::HashMap;

impl Solution {
    pub fn permute_unique(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut num_to_count: HashMap<i32, usize> = HashMap::new();
        nums.into_iter().for_each(|num| {
            let value = num_to_count.remove(&num).unwrap_or(0) + 1;
            num_to_count.insert(num, value);
        });
        solve_rec(num_to_count)
    }
}

fn solve_rec(num_to_count: HashMap<i32, usize>) -> Vec<Vec<i32>> {
    if num_to_count.is_empty() {
        return vec![vec![]];
    }
    let mut result: Vec<Vec<i32>> = Vec::new();
    for (&num, &count) in &num_to_count {
        let mut sub = num_to_count.clone();
        if count == 1 {
            sub.remove(&num);
        } else {
            sub.insert(num, count - 1);
        }

        let sub_result = solve_rec(sub);
        let mut sub_result: Vec<Vec<i32>> = sub_result
            .into_iter()
            .map(|mut v| {
                v.push(num);
                v
            })
            .collect();
        result.append(&mut sub_result);
    }
    result
}
// @lc code=end
