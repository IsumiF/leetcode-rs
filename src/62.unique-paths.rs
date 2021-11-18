/*
 * @lc app=leetcode id=62 lang=rust
 *
 * [62] Unique Paths
 */

struct Solution {}

// @lc code=start
use std::convert::TryInto;

impl Solution {

    // (m + n - 2)! / ((m - 1)! * (n - 1)!)
    // 8! / (6! * 2!)
    // 7 * 8 / 2

    pub fn unique_paths(m: i32, n: i32) -> i32 {
        let (m1, n1) = if m >= n { (m, n) } else { (n, m) };
        Self::unique_paths_impl(m1.try_into().unwrap(), n1.try_into().unwrap())
            .try_into()
            .unwrap()
    }

    fn unique_paths_impl(m: u64, n: u64) -> u64 {
        Self::product(m, m + n - 2) / Self::product(1, n - 1)
    }

    fn product(from: u64, to: u64) -> u64 {
        (from..(to + 1)).fold(1, |acc, x| acc * x)
    }
}

// @lc code=end

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn simple() {
        assert_eq!(Solution::unique_paths(3, 7), 28);
        assert_eq!(Solution::unique_paths(7, 3), 28);
        assert_eq!(Solution::unique_paths(3, 3), 6);
    }

    #[test]
    fn product() {
        assert_eq!(Solution::product(1, 3), 6);
        assert_eq!(Solution::product(2, 5), 120);
    }

    #[test]
    fn large() {
        assert_eq!(Solution::unique_paths(23, 12), 193536720);
    }
}
