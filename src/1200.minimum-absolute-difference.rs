/*
 * @lc app=leetcode id=1200 lang=rust
 *
 * [1200] Minimum Absolute Difference
 */

struct Solution {}

// @lc code=start
impl Solution {
    pub fn minimum_abs_difference(mut arr: Vec<i32>) -> Vec<Vec<i32>> {
        arr.sort();
        let min = (1..arr.len()).map(|i| arr[i] - arr[i - 1]).min().unwrap();
        (1..arr.len())
            .map(|i| (arr[i - 1], arr[i]))
            .filter(|&(x, y)| y - x == min)
            .map(|(x, y)| vec![x, y])
            .collect()
    }
}
// @lc code=end
