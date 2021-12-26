/*
 * @lc app=leetcode id=973 lang=rust
 *
 * [973] K Closest Points to Origin
 */

struct Solution {}

// @lc code=start
use std::collections::BinaryHeap;

impl Solution {
    pub fn k_closest(points: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        let mut points_heap = points
            .into_iter()
            .map(|p| Point::new(p))
            .collect::<BinaryHeap<Point>>();
        (0..k)
            .flat_map(|_| points_heap.pop())
            .map(|p| p.vec)
            .collect()
    }
}

#[derive(Eq, PartialEq)]
struct Point {
    vec: Vec<i32>,
    dist: i32,
}

impl Point {
    fn new(vec: Vec<i32>) -> Self {
        let x = vec[0];
        let y = vec[1];
        Self {
            vec,
            dist: x * x + y * y,
        }
    }
}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Point {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.dist.cmp(&other.dist).reverse()
    }
}

// @lc code=end
