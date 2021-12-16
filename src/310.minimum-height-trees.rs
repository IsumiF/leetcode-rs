/*
 * @lc app=leetcode id=310 lang=rust
 *
 * [310] Minimum Height Trees
 */

struct Solution {}

// @lc code=start
use std::collections::HashSet;

impl Solution {
    pub fn find_min_height_trees(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
        let mut solver = Solver::new(n as usize, edges);
        solver.solve().into_iter().map(|x| x as i32).collect()
    }
}

struct Solver {
    adj_list: Vec<HashSet<usize>>,
}

impl Solver {
    pub fn new(n: usize, edges: Vec<Vec<i32>>) -> Self {
        let mut adj_list: Vec<HashSet<usize>> = vec![HashSet::new(); n];
        for edge in edges {
            let i = edge[0] as usize;
            let j = edge[1] as usize;
            adj_list[i].insert(j);
            adj_list[j].insert(i);
        }
        Self { adj_list }
    }

    pub fn solve(&mut self) -> Vec<usize> {
        let mut leaves = HashSet::<usize>::new();
        for (i, adj) in self.adj_list.iter().enumerate() {
            if adj.len() == 1 {
                leaves.insert(i);
            }
        }

        let mut remaining: HashSet<usize> = HashSet::new();
        for i in 0..self.adj_list.len() {
            remaining.insert(i);
        }
        loop {
            if remaining.len() <= 2 {
                break;
            }
            let mut next_leaves = HashSet::<usize>::new();
            for &leaf in leaves.iter() {
                let adjs = std::mem::take(&mut self.adj_list[leaf]);
                for &adj in adjs.iter() {
                    self.adj_list[adj].remove(&leaf);
                    if self.adj_list[adj].len() == 1 {
                        next_leaves.insert(adj);
                    }
                }
                remaining.remove(&leaf);
            }
            leaves = next_leaves;
        }
        remaining.into_iter().collect()
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(4, vec![vec![1,0],vec![1,2],vec![1,3]] => vec![1])]
    #[test_case(6, vec![vec![3,0],vec![3,1],vec![3,2],vec![3,4],vec![5,4]] => vec![3, 4])]
    #[test_case(1, vec![] => vec![0])]
    #[test_case(2, vec![vec![0, 1]] => vec![0, 1])]
    #[test_case(6, vec![vec![0,1],vec![0,2],vec![0,3],vec![3,4],vec![4,5]] => vec![3])]
    fn test(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
        let mut got = Solution::find_min_height_trees(n, edges);
        got.sort();
        got
    }
}
