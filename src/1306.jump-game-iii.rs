/*
 * @lc app=leetcode id=1306 lang=rust
 *
 * [1306] Jump Game III
 */

struct Solution {}

// @lc code=start
impl Solution {
    pub fn can_reach(arr: Vec<i32>, start: i32) -> bool {
        let mut reachable = vec![false; arr.len()];
        std::iter::successors(Some(vec![start as usize]), |current_indicies| {
            if current_indicies.is_empty() {
                None
            } else {
                for &idx in current_indicies {
                    reachable[idx] = true;
                }
                Some(
                    current_indicies
                        .into_iter()
                        .flat_map(|&idx| {
                            let step = arr[idx];
                            vec![idx as i32 - step, idx as i32 + step]
                                .into_iter()
                                .filter(|&i| i >= 0 && i < arr.len() as i32)
                        })
                        .map(|idx| idx as usize)
                        .filter(|&idx| !reachable[idx])
                        .collect(),
                )
            }
        })
        .last();
        reachable
            .into_iter()
            .zip(arr.into_iter())
            .find(|&(ok, value)| ok && value == 0)
            .is_some()
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(vec![4,2,3,0,3,1,2], 5 => true)]
    #[test_case(vec![4,2,3,0,3,1,2], 0 => true)]
    #[test_case(vec![3,0,2,1,2], 2 => false)]
    fn test(arr: Vec<i32>, start: i32) -> bool {
        Solution::can_reach(arr, start)
    }
}
