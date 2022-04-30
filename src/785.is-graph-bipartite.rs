/*
 * @lc app=leetcode id=785 lang=rust
 *
 * [785] Is Graph Bipartite?
 */

struct Solution {}

// @lc code=start
use std::collections::HashSet;

impl Solution {
    pub fn is_bipartite(graph: Vec<Vec<i32>>) -> bool {
        let graph: Vec<Vec<usize>> = graph
            .into_iter()
            .map(|v| v.into_iter().map(|x| x as usize).collect())
            .collect();

        let mut visited_color = vec![false; graph.len()];
        let mut unvisited: HashSet<usize> = (0..graph.len()).collect();
        loop {
            let start_opt = unvisited.iter().next().copied();
            match start_opt {
                None => break,
                Some(start_idx) => {
                    let mut current_edge_points = HashSet::<usize>::new();
                    current_edge_points.insert(start_idx);
                    let mut color = true;
                    while !current_edge_points.is_empty() {
                        for &p in current_edge_points.iter() {
                            visited_color[p] = color;
                            unvisited.remove(&p);
                        }
                        current_edge_points = current_edge_points
                            .into_iter()
                            .flat_map(|p| {
                                graph[p]
                                    .iter()
                                    .filter(|&neighbor| unvisited.contains(neighbor))
                                    .map(|&neighbor| neighbor)
                            })
                            .collect();
                        color = !color;
                    }

                    visited_color[start_idx] = true;
                }
            }
        }

        for p in 0..graph.len() {
            let color = visited_color[p];
            if !graph[p].iter().all(|&n| visited_color[n] != color) {
                return false;
            }
        }
        true
    }
}

// @lc code=end
