/*
 * @lc app=leetcode id=56 lang=rust
 *
 * [56] Merge Intervals
 */

struct Solution {}

// @lc code=start
impl Solution {
    pub fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        Self::solve(intervals.into_iter().map(|v| (v[0], v[1])).collect())
            .map(|(x, y)| vec![x, y])
            .collect()
    }

    fn solve(mut intervals: Vec<(i32, i32)>) -> impl Iterator<Item = (i32, i32)> {
        intervals.sort_by_key(|x| x.0);
        intervals
            .into_iter()
            .fold(Vec::<(i32, i32)>::new(), |mut acc, cur| {
                let (to_push, push_cur) = acc
                    .pop()
                    .map(|prev| {
                        if cur.0 <= prev.1 {
                            ((prev.0, std::cmp::max(cur.1, prev.1)), false)
                        } else {
                            (prev, true)
                        }
                    })
                    .unwrap_or((cur, false));
                acc.push(to_push);
                if push_cur {
                    acc.push(cur);
                }
                acc
            })
            .into_iter()
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(vec![(1,3),(2,6),(8,10),(15,18)] => vec![(1, 6), (8,10), (15,18)])]
    #[test_case(vec![(1,4),(4,5)] => vec![(1,5)])]
    #[test_case(vec![(1,4),(2,3)] => vec![(1,4)])]
    fn test(intervals: Vec<(i32, i32)>) -> Vec<(i32, i32)> {
        Solution::merge(intervals.into_iter().map(|(x, y)| vec![x, y]).collect())
            .into_iter()
            .map(|xs| (xs[0], xs[1]))
            .collect()
    }
}
