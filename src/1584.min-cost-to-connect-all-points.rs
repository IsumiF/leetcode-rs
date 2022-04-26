/*
 * @lc app=leetcode id=1584 lang=rust
 *
 * [1584] Min Cost to Connect All Points
 */

struct Solution {}

// @lc code=start
use std::collections::HashMap;

impl Solution {
    pub fn min_cost_connect_points(points: Vec<Vec<i32>>) -> i32 {
        points
            .split_first()
            .map(|(first, remaining)| {
                let mut cluster = Vec::<Point>::new();
                cluster.reserve(points.len());
                let first = Point::new(first);
                cluster.push(first.clone());

                let mut rem_dist: HashMap<Point, i32> = remaining
                    .into_iter()
                    .map(|v| {
                        let p = Point::new(v);
                        (p.clone(), p.dist(&first))
                    })
                    .collect();

                let mut total_dist: i32 = 0;

                loop {
                    let min = rem_dist
                        .iter()
                        .min_by_key(|&(_, d)| *d)
                        .map(|(p, d)| (p.clone(), *d));
                    if let Some((p, d)) = min {
                        cluster.push(p.clone());
                        rem_dist.remove(&p);
                        total_dist += d;

                        rem_dist.iter_mut().for_each(|(p1, d)| {
                            let new_dist = p1.dist(&p);
                            if p1.dist(&p) < *d {
                                *d = new_dist;
                            }
                        })
                    } else {
                        break;
                    }
                }
                total_dist
            })
            .unwrap_or(0)
    }
}

#[derive(PartialEq, Eq, Hash, Clone)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(raw: &[i32]) -> Self {
        Self {
            x: raw[0],
            y: raw[1],
        }
    }

    fn dist(&self, other: &Point) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

// @lc code=end
