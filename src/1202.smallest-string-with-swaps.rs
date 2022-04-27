/*
 * @lc app=leetcode id=1202 lang=rust
 *
 * [1202] Smallest String With Swaps
 */

struct Solution {}

// @lc code=start
use std::{collections::HashMap, iter::FromIterator};

impl Solution {
    pub fn smallest_string_with_swaps(s: String, pairs: Vec<Vec<i32>>) -> String {
        let mut dsu = DSU::new(s.len());
        pairs.into_iter().for_each(|p| {
            let x = p[0] as usize;
            let y = p[1] as usize;
            dsu.union(x, y);
        });
        let mut groups: HashMap<usize, (Vec<usize>, Vec<char>)> = HashMap::new();
        s.chars()
            .enumerate()
            .map(|(idx, ch)| (dsu.find(idx), idx, ch))
            .for_each(|(group, idx, ch)| {
                let (mut v1, mut v2) = groups.remove(&group).unwrap_or((Vec::new(), Vec::new()));
                v1.push(idx);
                v2.push(ch);
                groups.insert(group, (v1, v2));
            });
        let mut result = vec!['0'; s.len()];
        groups.into_iter().for_each(|(_, (indicies, mut chars))| {
            chars.sort();
            indicies
                .into_iter()
                .zip(chars.into_iter())
                .for_each(|(idx, ch)| {
                    result[idx] = ch;
                });
        });
        String::from_iter(result.into_iter())
    }
}

struct DSU {
    // from set id to it's direct parent set's id
    par: Vec<usize>,
    // from set id to it's size
    size: Vec<usize>,
}

impl DSU {
    pub fn new(size: usize) -> Self {
        Self {
            par: (0..size).collect(),
            size: vec![1; size],
        }
    }

    pub fn find(&mut self, x: usize) -> usize {
        if self.par[x] != x {
            self.par[x] = self.find(self.par[x]);
        }
        self.par[x]
    }

    pub fn union(&mut self, x: usize, y: usize) {
        let xp = self.find(x);
        let yp = self.find(y);
        if self.size[xp] > self.size[yp] {
            self.par[yp] = self.par[xp];
            self.size[xp] += self.size[yp];
        } else {
            self.par[xp] = self.par[yp];
            self.size[yp] += self.size[xp];
        }
    }
}

// @lc code=end
