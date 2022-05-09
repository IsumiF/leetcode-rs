/*
 * @lc app=leetcode id=17 lang=rust
 *
 * [17] Letter Combinations of a Phone Number
 */

struct Solution {}

// @lc code=start
use std::collections::HashMap;

impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        let digit_to_letter: HashMap<char, &str> = vec![
            ('2', "abc"),
            ('3', "def"),
            ('4', "ghi"),
            ('5', "jkl"),
            ('6', "mno"),
            ('7', "pqrs"),
            ('8', "tuv"),
            ('9', "wxyz"),
        ]
        .into_iter()
        .collect();

        Self::comb_rec(&digit_to_letter, &digits)
            .into_iter()
            .map(|s| s.chars().rev().collect())
            .collect()
    }

    fn comb_rec(digit_to_letter: &HashMap<char, &str>, digits: &str) -> Vec<String> {
        if digits.is_empty() {
            return Vec::new();
        }
        let digit: char = digits.chars().next().unwrap();
        let (_, next_digits) = digits.split_at(1);
        let strs = Self::comb_rec(digit_to_letter, next_digits);
        if strs.is_empty() {
            digit_to_letter
                .get(&digit)
                .unwrap()
                .chars()
                .map(String::from)
                .collect()
        } else {
            strs.into_iter()
                .flat_map(|s| {
                    digit_to_letter.get(&digit).unwrap().chars().map(move |ch| {
                        let mut new_s = s.clone();
                        new_s.push(ch);
                        new_s
                    })
                })
                .collect()
        }
    }
}
// @lc code=end
