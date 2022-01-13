
/*
 * @lc app=leetcode id=452 lang=rust
 *
 * [452] Minimum Number of Arrows to Burst Balloons
 */

struct Solution {}

// @lc code=start

impl Solution {
    pub fn find_min_arrow_shots(mut points: Vec<Vec<i32>>) -> i32 {
        if points.is_empty() {
            return 0;
        }
        points.sort_by_key(|p| p[1]);

        let mut arrow_pos = points[0][1];
        let mut arrow_cnt = 1;
        for p in points {
            if arrow_pos >= p[0] {
                continue;
            }
            arrow_pos = p[1];
            arrow_cnt += 1;
        }
        arrow_cnt
    }
}

// @lc code=end
