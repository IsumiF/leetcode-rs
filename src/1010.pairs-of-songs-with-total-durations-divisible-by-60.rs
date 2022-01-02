/*
 * @lc app=leetcode id=1010 lang=rust
 *
 * [1010] Pairs of Songs With Total Durations Divisible by 60
 */

struct Solution {}

// @lc code=start
use std::collections::HashMap;

impl Solution {
    pub fn num_pairs_divisible_by60(mut time: Vec<i32>) -> i32 {
        for x in time.iter_mut() {
            *x = *x % 60;
        }
        let mut num_count: HashMap<i32, usize> = HashMap::new();
        for x in time {
            let new_count = num_count.remove(&x).unwrap_or(0) + 1;
            num_count.insert(x, new_count);
        }
        let zero_pairs = num_count
            .remove(&0)
            .and_then(|zeros| {
                if zeros >= 2 {
                    Some(zeros * (zeros - 1) / 2)
                } else {
                    None
                }
            })
            .unwrap_or(0);
        let diff_pairs: usize = num_count
            .iter()
            .flat_map(|(&num, &count)| {
                if num < 30 {
                    num_count.get(&(60 - num)).map(|&c2| count * c2)
                } else {
                    None
                }
            })
            .sum();
        let half_pairs: usize = num_count
            .remove(&30)
            .and_then(|halfs| {
                if halfs >= 2 {
                    Some(halfs * (halfs - 1) / 2)
                } else {
                    None
                }
            })
            .unwrap_or(0);
        (zero_pairs + diff_pairs + half_pairs) as i32
    }
}
// @lc code=end
