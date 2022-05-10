/*
 * @lc app=leetcode id=216 lang=rust
 *
 * [216] Combination Sum III
 */

struct Solution {}

// @lc code=start

impl Solution {
    pub fn combination_sum3(k: i32, n: i32) -> Vec<Vec<i32>> {
        let usable: Vec<i32> = (1..10).into_iter().collect();
        solve_rec(usable, k, n)
    }
}

fn solve_rec(mut usable: Vec<i32>, k: i32, n: i32) -> Vec<Vec<i32>> {
    if n < 0 {
        return vec![];
    }
    if k == 0 {
        if n == 0 {
            return vec![vec![]];
        }
        return vec![];
    }
    match usable.pop() {
        None => vec![],
        Some(x) => solve_rec(usable.clone(), k, n)
            .into_iter()
            .chain(
                solve_rec(usable, k - 1, n - x)
                    .into_iter()
                    .map(move |mut v| {
                        v.push(x);
                        v
                    }),
            )
            .collect(),
    }
}

// @lc code=end

// #[cfg(test)]
// mod tests {
//     use super::Solution;
//     use test_case::test_case;

//     #[test_case()]
//     fn test(k: i32, n: i32) -> Vec<Vec<i32>> {
//         Solution::combination_sum3()
//     }
// }
