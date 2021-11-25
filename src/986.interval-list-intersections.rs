/*
 * @lc app=leetcode id=986 lang=rust
 *
 * [986] Interval List Intersections
 */

struct Solution {}

// @lc code=start
use std::cmp;

impl Solution {
    pub fn interval_intersection(
        first_list: Vec<Vec<i32>>,
        second_list: Vec<Vec<i32>>,
    ) -> Vec<Vec<i32>> {
        let mut first_intervals = to_intervals(&first_list);
        let mut second_intervals = to_intervals(&second_list);
        match first_intervals
            .next()
            .and_then(|x| second_intervals.next().map(|y| (x, y)))
        {
            None => vec![],
            Some((mut left, mut right)) => {
                let mut result: Vec<Vec<i32>> = Vec::new();
                loop {
                    while left.entirely_less_than(&right) || right.entirely_less_than(&left) {
                        if left.entirely_less_than(&right) {
                            match first_intervals.next() {
                                None => return result,
                                Some(next) => left = next,
                            }
                        } else {
                            match second_intervals.next() {
                                None => return result,
                                Some(next) => right = next,
                            }
                        }
                    }
                    let left_bound = cmp::max(left.get_start(), right.get_start());
                    let right_bound = cmp::min(left.get_end(), right.get_end());
                    result.push(vec![left_bound, right_bound]);
                    if left.get_end() < right.get_end() {
                        match first_intervals.next() {
                            None => break,
                            Some(next) => left = next,
                        }
                    } else {
                        match second_intervals.next() {
                            None => break,
                            Some(next) => right = next,
                        }
                    }
                }

                result
            }
        }
    }
}

fn to_intervals<'a>(list: &'a Vec<Vec<i32>>) -> impl Iterator<Item = Interval<'a>> {
    list.iter().map(|vec| Interval::new(vec))
}

#[derive(Clone, Debug)]
struct Interval<'a> {
    raw: &'a [i32],
}

impl<'a> Interval<'a> {
    pub fn get_start(&self) -> i32 {
        self.raw[0]
    }

    pub fn get_end(&self) -> i32 {
        self.raw[1]
    }

    pub fn new(raw: &'a [i32]) -> Self {
        Self { raw }
    }

    pub fn intersection(&self, other: &Self) -> Option<(i32, i32)> {
        if self.get_end() >= other.get_start() && self.get_start() <= other.get_end() {
            let left_bound = cmp::max(self.get_start(), other.get_start());
            let right_bound = cmp::min(self.get_end(), other.get_end());
            Some((left_bound, right_bound))
        } else {
            None
        }
    }

    pub fn entirely_less_than(&self, other: &Self) -> bool {
        self.get_end() < other.get_start()
    }
}

// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn right_empty() {
        let want: Vec<Vec<i32>> = vec![];
        assert_eq!(
            Solution::interval_intersection(vec![vec![1, 3], vec![5, 9]], vec![]),
            want,
        )
    }

    #[test]
    fn left_empty() {
        let want: Vec<Vec<i32>> = vec![];
        assert_eq!(
            Solution::interval_intersection(vec![], vec![vec![4, 8], vec![10, 12]]),
            want,
        )
    }

    #[test]
    fn complex() {
        let want: Vec<Vec<i32>> = vec![
            vec![1, 2],
            vec![5, 5],
            vec![8, 10],
            vec![15, 23],
            vec![24, 24],
            vec![25, 25],
        ];
        assert_eq!(
            Solution::interval_intersection(
                vec![vec![0, 2], vec![5, 10], vec![13, 23], vec![24, 25]],
                vec![vec![1, 5], vec![8, 12], vec![15, 24], vec![25, 26]],
            ),
            want
        )
    }

    #[test]
    fn special() {
        let want: Vec<Vec<i32>> = vec![vec![8, 11]];
        assert_eq!(
            Solution::interval_intersection(
                vec![vec![4, 11]],
                vec![
                    vec![1, 2],
                    vec![8, 11],
                    vec![12, 13],
                    vec![14, 15],
                    vec![17, 19]
                ],
            ),
            want
        )
    }
}
