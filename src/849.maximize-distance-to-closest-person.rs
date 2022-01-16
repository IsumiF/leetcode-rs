/*
 * @lc app=leetcode id=849 lang=rust
 *
 * [849] Maximize Distance to Closest Person
 */

struct Solution {}

// @lc code=start
impl Solution {
    pub fn max_dist_to_closest(seats: Vec<i32>) -> i32 {
        let len = seats.len();
        let first_idx = seats
            .iter()
            .enumerate()
            .find(|&x| *x.1 == 1)
            .map(|x| x.0)
            .unwrap();
        let last_idx = seats
            .iter()
            .enumerate()
            .rev()
            .find(|&x| *x.1 == 1)
            .map(|x| x.0)
            .unwrap();
        let mid_dist = seats[first_idx..=last_idx]
            .iter()
            .enumerate()
            .fold((0, None), |(max, prev_idx), (cur_idx, &cur_val)| {
                if cur_val == 1 {
                    let dist = cur_idx - prev_idx.unwrap_or(0);
                    let mid_dist = dist / 2;
                    (std::cmp::max(max, mid_dist), Some(cur_idx))
                } else {
                    (max, prev_idx)
                }
            })
            .0;
        *[
            first_idx,
            len - last_idx - 1,
            mid_dist,
        ]
        .iter()
        .max()
        .unwrap() as i32
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(vec![1,0,0,0,1,0,1] => 2)]
    #[test_case(vec![1,0,0,0] => 3)]
    #[test_case(vec![0, 1] => 1)]
    fn test(seats: Vec<i32>) -> i32 {
        Solution::max_dist_to_closest(seats)
    }
}
