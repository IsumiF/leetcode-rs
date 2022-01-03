/*
 * @lc app=leetcode id=997 lang=rust
 *
 * [997] Find the Town Judge
 */

struct Solution {}

// @lc code=start
impl Solution {
    pub fn find_judge(n: i32, trust: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let mut adj_list: Vec<(Vec<usize>, Vec<usize>)> = vec![(vec![], vec![]); n];
        trust.into_iter().for_each(|t| {
            let from = t[0] as usize - 1;
            let to = t[1] as usize - 1;
            adj_list[from].0.push(to);
            adj_list[to].1.push(from);
        });
        adj_list
            .into_iter()
            .enumerate()
            .find_map(|(idx, (trusts, be_trusted_by))| {
                if trusts.is_empty() && be_trusted_by.len() == n - 1 {
                    Some(idx)
                } else {
                    None
                }
            })
            .map(|idx| idx as i32 + 1)
            .unwrap_or(-1)
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(2, vec![(1, 2)] => 2)]
    #[test_case(3, vec![(1, 3), (2, 3)] => 3)]
    #[test_case(3, vec![(1, 3), (2, 3), (3, 1)] => -1)]
    fn test(n: i32, trust: Vec<(i32, i32)>) -> i32 {
        Solution::find_judge(n, trust.into_iter().map(|(x, y)| vec![x, y]).collect())
    }
}
