/*
 * @lc app=leetcode id=844 lang=rust
 *
 * [844] Backspace String Compare
 */

struct Solution {}

// @lc code=start
impl Solution {
    pub fn backspace_compare(s: String, t: String) -> bool {
        run_string(s) == run_string(t)
    }
}

fn run_string(s: String) -> String {
    s.chars().fold(String::new(), |mut acc, ch| {
        if ch == '#' {
            acc.pop();
        } else {
            acc.push(ch);
        }
        acc
    })
}

// @lc code=end
