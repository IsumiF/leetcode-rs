/*
 * @lc app=leetcode id=3 lang=rust
 *
 * [3] Longest Substring Without Repeating Characters
 */

struct Solution {}

// @lc code=start
use std::collections::HashSet;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let chars: Vec<char> = s.chars().collect();
        if chars.is_empty() {
            return 0;
        }
        let mut current_chars = HashSet::<char>::new();
        let mut j = 0usize;
        let mut max_len = 0usize;
        current_chars.insert(chars[0]);
        for i in 0..chars.len() {
            loop {
                if j + 1 >= chars.len() {
                    break;
                }
                if current_chars.contains(&chars[j + 1]) {
                    break;
                } else {
                    current_chars.insert(chars[j + 1]);
                    j += 1;
                }
            }
            if current_chars.len() > max_len {
                max_len = current_chars.len();
            }
            current_chars.remove(&chars[i]);
            if j < i {
                j = i;
            }
        }
        max_len as i32
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::Solution;
    use test_case::test_case;

    #[test_case("abcabcbb" => 3)]
    fn test(s: &str) -> i32 {
        Solution::length_of_longest_substring(String::from(s))
    }
}
