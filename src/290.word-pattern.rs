/*
 * @lc app=leetcode id=290 lang=rust
 *
 * [290] Word Pattern
 */

struct Solution {}

// @lc code=start
use std::collections::HashMap;

impl Solution {
    pub fn word_pattern(pattern: String, s: String) -> bool {
        let mut pattern_map: HashMap<char, &str> = HashMap::new();
        let mut pattern_map_rev: HashMap<&str, char> = HashMap::new();
        let mut words = s.split_ascii_whitespace();
        for ch in pattern.chars() {
            match words.next() {
                None => return false,
                Some(word) => match pattern_map.get(&ch) {
                    Some(&w) => {
                        if w != word {
                            return false;
                        }
                    }
                    None => {
                        if pattern_map_rev.get(word).is_some() {
                            return false;
                        } else {
                            pattern_map.insert(ch, word);
                            pattern_map_rev.insert(word, ch);
                        }
                    }
                },
            }
        }
        if words.next().is_some() {
            return false;
        }
        true
    }
}
// @lc code=end
