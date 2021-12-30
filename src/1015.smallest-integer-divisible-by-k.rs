/*
 * @lc app=leetcode id=1015 lang=rust
 *
 * [1015] Smallest Integer Divisible by K
 */

struct Solution {}

// @lc code=start
impl Solution {
    pub fn smallest_repunit_div_by_k(k: i32) -> i32 {
        if k % 2 == 0 || k % 5 == 0 {
            -1
        } else {
            let (_, len) = std::iter::successors(Some((1, 1)), |&(x, len)| {
                if x % k == 0 {
                    None
                } else {
                    Some(((x * 10 + 1) % k, len+1))
                }
            }).last().unwrap();
            len
        }
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(1 => 1)]
    #[test_case(2 => -1)]
    #[test_case(3 => 3)]
    fn test(k: i32) -> i32 {
        Solution::smallest_repunit_div_by_k(k)
    }
}