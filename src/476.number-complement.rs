/*
 * @lc app=leetcode id=476 lang=rust
 *
 * [476] Number Complement
 */

struct Solution {}

// @lc code=start
impl Solution {
    pub fn find_complement(num: i32) -> i32 {
        let num_bits = count_bits(num);
        let mut mask = 0;
        for _ in 0..num_bits {
            mask = mask << 1;
            mask += 1;
        }
        println!("mask: {}", mask);
        mask & (!num)
    }
}

fn count_bits(mut num: i32) -> usize {
    let mut count = 0usize;
    while num > 0 {
        num = num >> 1;
        count += 1;
    }
    count
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(5 => 2)]
    #[test_case(1 => 0)]
    fn test(num: i32) -> i32 {
        Solution::find_complement(num)
    }
}
