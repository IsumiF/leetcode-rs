/*
 * @lc app=leetcode id=210 lang=rust
 *
 * [210] Course Schedule II
 */

struct Solution {}

// @lc code=start
use std::collections::HashSet;

impl Solution {
    pub fn find_order(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> Vec<i32> {
        let n = num_courses as usize;
        // course -> (pre, post)
        let mut adj_list = prerequisites.into_iter().fold(
            vec![(HashSet::<usize>::new(), Vec::<usize>::new()); n],
            |mut acc, val| {
                let i = val[0] as usize;
                let j = val[1] as usize;
                acc[i].0.insert(j);
                acc[j].1.push(i);
                acc
            },
        );

        let mut courses_no_prereq =
            adj_list
                .iter()
                .enumerate()
                .fold(Vec::<usize>::new(), |mut acc, (course, (pre, _))| {
                    if pre.is_empty() {
                        acc.push(course);
                    }
                    acc
                });

        let mut ordered_list = Vec::<i32>::new();
        ordered_list.reserve_exact(n);
        while !courses_no_prereq.is_empty() {
            ordered_list.extend(courses_no_prereq.iter().map(|&x| x as i32));
            let mut new_courses_no_prereq = Vec::<usize>::new();
            for &c in &courses_no_prereq {
                let (_, post) = std::mem::take(&mut adj_list[c]);
                for i in post {
                    adj_list[i].0.remove(&c);
                    if adj_list[i].0.is_empty() {
                        new_courses_no_prereq.push(i);
                    }
                }
            }
            courses_no_prereq = new_courses_no_prereq;
        }
        if ordered_list.len() == n {
            ordered_list
        } else {
            Vec::new()
        }
    }
}

// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(2, vec![vec![1, 0]] => vec![0, 1])]
    #[test_case(4, vec![vec![1, 0], vec![2, 0], vec![3, 1], vec![3, 2]] => vec![0, 1, 2, 3])]
    #[test_case(1, vec![] => vec![0])]
    fn test(n: i32, pre: Vec<Vec<i32>>) -> Vec<i32> {
        Solution::find_order(n, pre)
    }
}
