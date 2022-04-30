/*
 * @lc app=leetcode id=399 lang=rust
 *
 * [399] Evaluate Division
 */

struct Solution {}

// @lc code=start
use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn calc_equation(
        equations: Vec<Vec<String>>,
        values: Vec<f64>,
        queries: Vec<Vec<String>>,
    ) -> Vec<f64> {
        let mut adj_list: HashMap<String, HashMap<String, f64>> = equations
            .into_iter()
            .zip(values.into_iter())
            .flat_map(|(mut eq, v)| {
                let to = eq.pop().unwrap();
                let from = eq.pop().unwrap();
                [
                    (from.clone(), to.clone(), v),
                    (to.clone(), from.clone(), 1f64 / v),
                    (from.clone(), from, 1f64),
                    (to.clone(), to, 1f64),
                ]
            })
            .fold(HashMap::new(), |mut result, (from, to, v)| {
                let mut entry = result.remove(&from).unwrap_or(HashMap::new());
                entry.insert(to, v);
                result.insert(from, entry);
                result
            });

        queries
            .into_iter()
            .map(|mut query| {
                let to = query.pop().unwrap();
                let from = query.pop().unwrap();

                let mut visited = HashSet::new();
                search(&mut visited, &mut adj_list, from, to).unwrap_or(-1f64)
            })
            .collect()
    }
}

fn search(
    visited: &mut HashSet<String>,
    adj_list: &mut HashMap<String, HashMap<String, f64>>,
    current: String,
    target: String,
) -> Option<f64> {
    // println!("current: {}, target: {}", current, target);
    if let Some(v) = adj_list
        .get(&current)
        .and_then(|neighbors| neighbors.get(&target))
    {
        return Some(*v);
    }

    visited.insert(current.clone());
    let nexts = adj_list.get(&current)?.clone();
    nexts.into_iter().find_map(|(next, value)| {
        if visited.contains(&next) {
            return None;
        }
        let next = next.clone();
        let next_to_target = search(visited, adj_list, next.clone(), target.clone())?;
        let mut m = adj_list.remove(&next).unwrap_or(HashMap::new());
        m.insert(target.clone(), next_to_target);
        adj_list.insert(next, m);
        Some(next_to_target * value)
    })
}

// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(vec![vec!["a","b"],vec!["b","c"]], vec![2.0, 3.0], vec![vec!["a","c"],vec!["b","a"],vec!["a","e"],vec!["a","a"],vec!["x","x"]] => vec![6.00000,0.50000,-1.00000,1.00000,-1.00000])]
    fn test(equations: Vec<Vec<&str>>, values: Vec<f64>, queries: Vec<Vec<&str>>) -> Vec<f64> {
        Solution::calc_equation(
            equations
                .into_iter()
                .map(|v| v.into_iter().map(String::from).collect())
                .collect(),
            values,
            queries
                .into_iter()
                .map(|v| v.into_iter().map(String::from).collect())
                .collect(),
        )
    }
}
