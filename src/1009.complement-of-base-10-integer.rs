/*
 * @lc app=leetcode id=1009 lang=rust
 *
 * [1009] Complement of Base 10 Integer
 */

struct Solution {}

// @lc code=start
impl Solution {
    pub fn bitwise_complement(num: i32) -> i32 {
        if num == 0 {
            1
        } else {
            let num_bits = count_bits(num);
            let mut mask = 0;
            for _ in 0..num_bits {
                mask = mask << 1;
                mask += 1;
            }
            mask & (!num)
        }
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
