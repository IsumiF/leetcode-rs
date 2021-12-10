/*
 * @lc app=leetcode id=790 lang=rust
 *
 * [790] Domino and Tromino Tiling
 */

struct Solution {}

// @lc code=start
impl Solution {
    pub fn num_tilings(n: i32) -> i32 {
        let mut f: Vec<i64> = vec![0; std::cmp::max(n as usize + 1, 5)];
        f[1] = 1;
        f[2] = 2;
        f[3] = 5;
        f[4] = 11;
        let modulo = 1000000007;

        for i in 5..=n as usize {
            f[i] = (2 * f[i - 1] + f[i - 3]) % modulo;
        }
        f[n as usize] as i32
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(5 => 24)]
    #[test_case(30 => 312342182)]
    fn test(n: i32) -> i32 {
        Solution::num_tilings(n)
    }
}
