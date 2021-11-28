/*
 * @lc app=leetcode id=797 lang=rust
 *
 * [797] All Paths From Source to Target
 */

struct Solution {}

// @lc code=start
impl Solution {
    pub fn all_paths_source_target(graph: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let graph_usize: Vec<Vec<usize>> = graph
            .into_iter()
            .map(|v| v.into_iter().map(|x| x as usize).collect())
            .collect();
        Self::all_paths_source_target_rec(&graph_usize, 0)
            .into_iter()
            .map(|mut v| {
                v.reverse();
                v.into_iter().map(|x| x as i32).collect()
            })
            .collect()
    }

    fn all_paths_source_target_rec(graph: &Vec<Vec<usize>>, from_index: usize) -> Vec<Vec<usize>> {
        if from_index == graph.len() - 1 {
            vec![vec![from_index]]
        } else {
            let next_indicies = &graph[from_index];
            let paths: Vec<Vec<usize>> = next_indicies
                .iter()
                .flat_map(|&next_index| {
                    Self::all_paths_source_target_rec(graph, next_index).into_iter()
                })
                .map(|mut path| {
                    path.push(from_index);
                    path
                })
                .collect();
            paths
        }
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(vec![vec![1, 2], vec![3], vec![3], vec![]] => vec![vec![0, 1, 3], vec![0, 2, 3]])]
    #[test_case(vec![vec![4,3,1],vec![3,2,4],vec![3],vec![4],vec![]] =>
        vec![vec![0,4], vec![0,3,4], vec![0,1,3,4], vec![0,1,2,3,4], vec![0,1,4]])]
    #[test_case(vec![vec![1], vec![]] => vec![vec![0, 1]])]
    #[test_case(vec![vec![1, 2, 3], vec![2], vec![3], vec![]] => vec![vec![0, 1, 2, 3], vec![0, 2, 3], vec![0, 3]])]
    #[test_case(vec![vec![1, 3], vec![2], vec![3], vec![]] => vec![vec![0, 1, 2, 3], vec![0, 3]])]
    fn test(graph: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        Solution::all_paths_source_target(graph)
    }
}
