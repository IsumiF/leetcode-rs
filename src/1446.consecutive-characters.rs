/*
 * @lc app=leetcode id=1446 lang=rust
 *
 * [1446] Consecutive Characters
 */

struct Solution {}

// @lc code=start
impl Solution {
    pub fn max_power(s: String) -> i32 {
        let mut prev_ch: Option<char> = None;
        let mut max_count: i32 = 0;
        let mut count: i32 = 0;
        for ch in s.chars() {
            if Some(ch) == prev_ch {
                count += 1;
            } else {
                prev_ch = Some(ch);
                if count > max_count {
                    max_count = count;
                }
                count = 1;
            }
        }
        std::cmp::max(count, max_count)
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("leetcode" => 2)]
    #[test_case("abbcccddddeeeeedcba" => 5)]
    #[test_case("triplepillooooow" => 5)]
    #[test_case("hooraaaaaaaaaaay" => 11)]
    #[test_case("tourist" => 1)]
    fn test(s: &str) -> i32 {
        Solution::max_power(s.to_string())
    }
}
