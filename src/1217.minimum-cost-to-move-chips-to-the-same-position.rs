/*
 * @lc app=leetcode id=1217 lang=rust
 *
 * [1217] Minimum Cost to Move Chips to The Same Position
 */

struct Solution {}

// @lc code=start
impl Solution {
    pub fn min_cost_to_move_chips(position: Vec<i32>) -> i32 {
        let even_count = position
            .iter()
            .fold(0usize, |acc, val| if val % 2 == 0 { acc + 1 } else { acc });
        std::cmp::min(even_count, position.len() - even_count) as i32
    }
}
// @lc code=end
