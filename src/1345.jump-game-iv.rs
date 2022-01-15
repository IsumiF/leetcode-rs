/*
 * @lc app=leetcode id=1345 lang=rust
 *
 * [1345] Jump Game IV
 */

struct Solution {}

// @lc code=start
use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn min_jumps(arr: Vec<i32>) -> i32 {
        let mut value_to_indicies = HashMap::<i32, Vec<usize>>::new();
        for (i, &v) in arr.iter().enumerate() {
            let mut indicies = value_to_indicies.remove(&v).unwrap_or(Vec::<usize>::new());
            indicies.push(i);
            value_to_indicies.insert(v, indicies);
        }

        let mut dist: Vec<i32> = vec![-1; arr.len()];
        let mut current_indicies = HashSet::<usize>::new();
        current_indicies.insert(0);
        let mut cur_dist = 0;
        while !current_indicies.is_empty() {
            let mut next_indicies = HashSet::<usize>::new();
            for &idx in current_indicies.iter() {
                dist[idx] = cur_dist;
                if idx == arr.len() - 1 {
                    return dist[idx];
                }
            }
            for &idx in current_indicies.iter() {
                if idx >= 1 && dist[idx - 1] == -1 {
                    next_indicies.insert(idx - 1);
                }
                if arr.len() >= 2 && idx <= arr.len() - 2 && dist[idx + 1] == -1 {
                    next_indicies.insert(idx + 1);
                }
                for &next in value_to_indicies
                    .get(&arr[idx])
                    .unwrap_or(&Vec::<usize>::new())
                {
                    if dist[next] == -1 {
                        next_indicies.insert(next);
                    }
                }
                value_to_indicies.remove(&arr[idx]);
            }
            cur_dist += 1;
            current_indicies = next_indicies;
        }
        *dist.last().unwrap()
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(vec![100,-23,-23,404,100,23,23,23,3,404] => 3)]
    #[test_case(vec![7] => 0)]
    #[test_case(vec![7,6,9,6,9,6,9,7] => 1)]
    fn test(arr: Vec<i32>) -> i32 {
        Solution::min_jumps(arr)
    }
}
