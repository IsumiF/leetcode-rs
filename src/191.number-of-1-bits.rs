/*
 * @lc app=leetcode id=191 lang=rust
 *
 * [191] Number of 1 Bits
 */

struct Solution {}

// @lc code=start
impl Solution {
    pub fn hammingWeight(mut n: u32) -> i32 {
        let mut count = 0;
        while n > 0 {
            if n & 1 == 1 {
                count += 1;
            }
            n = n >> 1;
        }
        count
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::Solution;
    use test_case::test_case;

    #[test_case(0b00000000000000000000000000001011 => 3)]
    fn test(n: u32) -> i32 {
        Solution::hammingWeight(n)
    }
}