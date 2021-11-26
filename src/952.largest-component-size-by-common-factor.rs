/*
 * @lc app=leetcode id=952 lang=rust
 *
 * [952] Largest Component Size by Common Factor
 *
 * https://leetcode.com/problems/largest-component-size-by-common-factor/discuss/1592093/Daily-LeetCoding-Challenge-November-Day-23
 * https://leetcode.com/problems/largest-component-size-by-common-factor/discuss/1592263/C%2B%2B-Simple-Solution-w-Explanation-or-Disjoint-Set-Union-%2B-Sieve-of-Eratosthenes
 *
 */

struct Solution {}

// @lc code=start
use std::collections::HashMap;

impl Solution {
    pub fn largest_component_size(nums: Vec<i32>) -> i32 {
        if let Some(max_num) = nums.iter().max() {
            let mut dsu = DSU::new((max_num + 1) as usize);
            let mut parent_to_count: HashMap<usize, i32> = HashMap::new();
            for num in nums.iter() {
                let sqrt = (*num as f64).sqrt() as i32;
                for factor in 2..=sqrt {
                    if num % factor == 0 {
                        dsu.union(*num as usize, (num / factor) as usize);
                        dsu.union(*num as usize, factor as usize);
                    }
                }
            }

            for num in nums.iter() {
                let par = dsu.find(*num as usize);
                parent_to_count.insert(par, parent_to_count.get(&par).unwrap_or(&0) + 1);
            }
            parent_to_count
                .iter()
                .map(|(_, &count)| count)
                .max()
                .unwrap_or(0)
        } else {
            0
        }
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

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(vec![4, 6, 15, 35] => 4)]
    #[test_case(vec![20, 50, 9, 63] => 2)]
    #[test_case(vec![2,3,6,7,4,12,21,39] => 8)]
    fn test(nums: Vec<i32>) -> i32 {
        Solution::largest_component_size(nums)
    }
}
