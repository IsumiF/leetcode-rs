/*
 * @lc app=leetcode id=318 lang=rust
 *
 * [318] Maximum Product of Word Lengths
 */

struct Solution {}

// @lc code=start
impl Solution {
    pub fn max_product(words: Vec<String>) -> i32 {
        let words_hash: Vec<(usize, u32)> = words
            .into_iter()
            .map(|w| (w.len(), hash_word(&w)))
            .collect();
        let mut max_product = 0usize;
        for i in 0..words_hash.len() {
            for j in i + 1..words_hash.len() {
                let &(len1, h1) = &words_hash[i];
                let &(len2, h2) = &words_hash[j];
                if h1 & h2 == 0 && len1 * len2 > max_product {
                    max_product = len1 * len2;
                }
            }
        }
        max_product as i32
    }
}

fn hash_word(word: &str) -> u32 {
    word.chars()
        .map(|ch| 1 << (ch as u32 - 'a' as u32))
        .fold(0u32, |acc, val| acc | val)
}

// @lc code=end

#[cfg(test)]
mod tests {
    use super::Solution;
    use test_case::test_case;

    #[test_case(vec!["abcw","baz","foo","bar","xtfn","abcdef"] => 16)]
    fn test(words: Vec<&str>) -> i32 {
        Solution::max_product(words.into_iter().map(String::from).collect())
    }
}
