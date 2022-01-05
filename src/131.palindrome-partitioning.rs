/*
 * @lc app=leetcode id=131 lang=rust
 *
 * [131] Palindrome Partitioning
 */

struct Solution {}

// @lc code=start
use std::{
    cell::RefCell,
    collections::{HashMap, HashSet},
};

impl Solution {
    pub fn partition(s: String) -> Vec<Vec<String>> {
        let solver = Solver::new();
        let chars: Vec<char> = s.chars().collect();
        let result = solver.solve_rec(&chars);
        result
            .into_iter()
            .map(|v| v.into_iter().map(|s| s.iter().collect()).collect())
            .collect()
    }
}

struct Solver<'a> {
    cache: RefCell<HashMap<&'a [char], Vec<Vec<&'a [char]>>>>,
}

impl<'a> Solver<'a> {
    fn new() -> Self {
        Self {
            cache: RefCell::new(HashMap::new()),
        }
    }

    fn solve_rec(&self, s: &'a [char]) -> Vec<Vec<&'a [char]>> {
        if s.len() == 0 {
            return Vec::new();
        }
        if s.len() == 1 {
            return vec![vec![s]];
        }
        let cached_value = self.cache.borrow().get(&s).cloned();
        match cached_value {
            Some(v) => v,
            None => {
                let mut result = HashSet::<Vec<&'a [char]>>::new();
                if is_palindrome(s) {
                    result.insert(vec![s]);
                }
                for i in 1..s.len() {
                    let (left, right) = s.split_at(i);
                    let left_result = self.solve_rec(left);
                    let right_result = self.solve_rec(right);
                    for left_case in left_result {
                        for right_case in right_result.iter() {
                            let mut comb_case = left_case.clone();
                            comb_case.extend(right_case);
                            result.insert(comb_case);
                        }
                    }
                }
                let v: Vec<Vec<&[char]>> = result.into_iter().collect();
                self.cache.borrow_mut().insert(&s, v.clone());
                v
            }
        }
    }
}

fn is_palindrome(s: &[char]) -> bool {
    for i in 0..s.len() {
        if s[i] != s[s.len() - i - 1] {
            return false;
        }
    }
    true
}

// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("aab" => vec![vec!["a","a","b"],vec!["aa","b"]])]
    #[test_case("a" => vec![vec!["a"]])]
    fn test(s: &str) -> Vec<Vec<String>> {
        Solution::partition(s.to_string())
    }
}
