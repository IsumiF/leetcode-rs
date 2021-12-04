/*
 * @lc app=leetcode id=84 lang=rust
 *
 * [84] Largest Rectangle in Histogram
 */

struct Solution {}

// @lc code=start
impl Solution {
    pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
        let mut stack = Vec::<usize>::new();
        stack.reserve_exact(heights.len());
        let mut max_area: i32 = i32::MIN;
        let mut pop = |stack: &mut Vec<usize>, right_index: usize| -> bool {
            if let Some(cur_idx) = stack.pop() {
                let left_index = stack.last().map(|&left_index| left_index + 1).unwrap_or(0);
                let cur_area = (right_index - left_index + 1) as i32 * heights[cur_idx];
                if cur_area > max_area {
                    max_area = cur_area;
                }
                true
            } else {
                false
            }
        };

        for (idx, &height) in heights.iter().enumerate() {
            loop {
                if !stack
                    .last()
                    .map(|&prev_idx| heights[prev_idx] < height)
                    .unwrap_or(true)
                {
                    pop(&mut stack, idx - 1);
                } else {
                    break;
                }
            }
            stack.push(idx);
        }
        while !stack.is_empty() {
            pop(&mut stack, heights.len() - 1);
        }
        max_area
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(vec![2, 1, 5, 6, 2, 3] => 10)]
    #[test_case(vec![2, 4] => 4)]
    fn test(heights: Vec<i32>) -> i32 {
        Solution::largest_rectangle_area(heights)
    }
}
