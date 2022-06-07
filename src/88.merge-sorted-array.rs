/*
 * @lc app=leetcode id=88 lang=rust
 *
 * [88] Merge Sorted Array
 */

struct Solution {}

// @lc code=start
impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let mut nums1_iter = nums1.iter().take(m as usize).peekable();
        let mut nums2_iter = nums2.iter().take(n as usize).peekable();
        let mut merged = Vec::<i32>::new();
        loop {
            let x = nums1_iter.peek().copied().copied().unwrap_or(i32::MAX);
            let y = nums2_iter.peek().copied().copied().unwrap_or(i32::MAX);
            if x == i32::MAX && y == i32::MAX {
                break;
            }
            if x <= y {
                nums1_iter.next();
            } else {
                nums2_iter.next();
            }
            merged.push(x.min(y))
        }
        *nums1 = merged;
    }
}
// @lc code=end
