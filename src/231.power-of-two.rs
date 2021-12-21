/*
 * @lc app=leetcode id=231 lang=rust
 *
 * [231] Power of Two
 */

struct Solution {}

// @lc code=start
impl Solution {
    pub fn is_power_of_two(n: i32) -> bool {
        std::iter::successors(Some(1), |&prev| {
            if prev == 1073741824 {
                None
            } else {
                Some(prev * 2)
            }
        })
        .find(|&x| x == n)
        .is_some()
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(-2147483648 => false)]
    #[test_case(-16 => false)]
    fn test(n: i32) -> bool {
        Solution::is_power_of_two(n)
    }
}
