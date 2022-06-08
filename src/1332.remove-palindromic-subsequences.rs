/*
 * @lc app=leetcode id=1332 lang=rust
 *
 * [1332] Remove Palindromic Subsequences
 */

struct Solution {}

// @lc code=start
impl Solution {
    pub fn remove_palindrome_sub(s: String) -> i32 {
        if Self::is_palindrome(&s) {
            1
        } else {
            2
        }
    }

    fn is_palindrome(s: &str) -> bool {
        let chars: Vec<char> = s.chars().collect();
        for i in 0..chars.len() / 2 {
            if chars[i] != chars[chars.len() - 1 - i] {
                return false;
            }
        }
        true
    }
}
// @lc code=end
