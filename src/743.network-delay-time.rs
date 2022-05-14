/*
 * @lc app=leetcode id=743 lang=rust
 *
 * [743] Network Delay Time
 */

struct Solution {}

// @lc code=start

use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap},
};

impl Solution {
    pub fn network_delay_time(times: Vec<Vec<i32>>, n: i32, k: i32) -> i32 {
        let mut adj_mat: HashMap<i32, Vec<(i32, i32)>> = HashMap::new();
        for tm in times {
            let source = tm[0];
            let dest = tm[1];
            let time = tm[2];
            let mut row: Vec<(i32, i32)> = adj_mat.remove(&source).unwrap_or(Vec::new());
            row.push((dest, time));
            adj_mat.insert(source, row);
        }
        let mut unvisited = BinaryHeap::<UnvisitedNode>::new();
        unvisited.push(UnvisitedNode { node: k, cost: 0 });
        let mut node_cost: HashMap<i32, i32> = (1..=n)
            .into_iter()
            .map(|node| (node, if node == k { 0 } else { i32::MAX }))
            .collect();
        loop {
            match unvisited.pop() {
                None => break,
                Some(cur) => {
                    if cur.cost == i32::MAX {
                        break;
                    }
                    for &(neighbor, cur_to_neighbor) in
                        adj_mat.get(&cur.node).unwrap_or(&Vec::new())
                    {
                        let new_cost = cur.cost + cur_to_neighbor;
                        let old_cost = *node_cost.get(&neighbor).unwrap();
                        if new_cost < old_cost {
                            node_cost.insert(neighbor, new_cost);
                            unvisited.push(UnvisitedNode { node: neighbor, cost: new_cost });
                        }
                    }
                }
            }
        }

        let max_dist = node_cost.into_iter().map(|(_, c)| c).max().unwrap();
        if max_dist == i32::MAX {
            -1
        } else {
            max_dist
        }
    }
}

#[derive(Clone, Copy)]
struct UnvisitedNode {
    node: i32,
    cost: i32,
}

impl PartialEq for UnvisitedNode {
    fn eq(&self, other: &Self) -> bool {
        self.cost == other.cost
    }
}

impl Eq for UnvisitedNode {}

impl PartialOrd for UnvisitedNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for UnvisitedNode {
    fn cmp(&self, other: &Self) -> Ordering {
        self.cost.cmp(&other.cost).reverse()
    }
}

// @lc code=end

#[cfg(test)]
mod tests {
    use super::Solution;
    use test_case::test_case;

    #[test_case(vec![vec![2, 1, 1], vec![2, 3, 1], vec![3,4,1]], 4, 2 => 2)]
    fn test(times: Vec<Vec<i32>>, n: i32, k: i32) -> i32 {
        Solution::network_delay_time(times, n, k)
    }
}
