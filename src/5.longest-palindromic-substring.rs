
struct Solution {}

use std::iter::FromIterator;

impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let chars: Vec<char> = s.chars().collect();
        let mut dp = vec![vec![false; chars.len()]; chars.len()];
        for i in 0..chars.len() {
            dp[i][i] = true;
        }
        for i in 0..chars.len() - 1 {
            dp[i][i+1] = chars[i] == chars[i+1];
        }
        for step in 2..chars.len() - 1 {
            for i in 0..chars.len() - step - 1 {
                let j = i + step;
                dp[i][j] = dp[i+1][j-1] && chars[i] == chars[j];
            }
        }
        for step in (0..chars.len() - 1).rev() {
            for i in 0..chars.len() - step - 1 {
                if dp[i][i+step] {
                    return chars[i..i+step+1].iter().collect::<String>();
                }
            }
        }

        panic!("not found")
    }
}

