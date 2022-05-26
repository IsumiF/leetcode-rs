/*
 * @lc app=leetcode id=474 lang=rust
 *
 * [474] Ones and Zeroes
 */

struct Solution {}

// @lc code=start
use std::collections::HashMap;

impl Solution {
    pub fn find_max_form(strs: Vec<String>, m: i32, n: i32) -> i32 {
        let elems: Vec<(i32, i32)> = strs
            .into_iter()
            .map(|str| {
                let zeros = str.chars().filter(|&ch| ch == '0').count() as i32;
                let ones = str.chars().filter(|&ch| ch == '1').count() as i32;
                return (zeros, ones);
            })
            .collect();

        todo!()
    }
}

struct Solver {
    
}

impl Solver {
    fn solve(elems_start: usize, max_zeros: i32, max_ones: i32) {

    }
}

// @lc code=end
