/*
 * @lc app=leetcode id=605 lang=rust
 *
 * [605] Can Place Flowers
 */

struct Solution {}

// @lc code=start
impl Solution {
    pub fn can_place_flowers(flowerbed: Vec<i32>, n: i32) -> bool {
        let mut dist = 1usize;
        let mut max_n = 0usize;
        for &val in flowerbed.iter() {
            if val == 1 {
                max_n += (dist - 1) / 2;
                if max_n >= n as usize {
                    return true;
                }
                dist = 0;
            } else {
                dist += 1;
            }
        }
        if dist > 0 {
            max_n += dist / 2;
        }
        n as usize <= max_n
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(vec![1, 0, 0, 0, 1], 1 => true)]
    #[test_case(vec![1, 0, 0, 0, 1], 2 => false)]
    #[test_case(vec![0, 0, 1, 0, 1], 1 => true)]
    #[test_case(vec![0, 0, 1, 0, 1], 2 => false)]
    #[test_case(vec![0, 0, 0, 1, 0, 1], 1 => true)]
    #[test_case(vec![0, 0, 0, 1, 0, 1], 2 => false)]
    fn test(flowerbed: Vec<i32>, n: i32) -> bool {
        Solution::can_place_flowers(flowerbed, n)
    }
}
