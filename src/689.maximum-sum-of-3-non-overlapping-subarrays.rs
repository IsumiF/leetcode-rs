/*
 * @lc app=leetcode id=689 lang=rust
 *
 * [689] Maximum Sum of 3 Non-Overlapping Subarrays
 */

struct Solution {}

// @lc code=start
#[derive(Clone, Debug)]
struct Acc2 {
    indicies: [usize; 2],
    sum: i32,
}

#[derive(Clone, Debug)]
struct Acc3 {
    indicies: [usize; 3],
    sum: i32,
}

impl Solution {
    pub fn max_sum_of_three_subarrays(nums: Vec<i32>, ki32: i32) -> Vec<i32> {
        let k = ki32 as usize;
        let mut sums = vec![0; nums.len() - (k - 1)];
        for i in 0usize..k {
            sums[0] += nums[i];
        }
        for i in 1usize..sums.len() {
            let j = i + k - 1;
            sums[i] = sums[i - 1] - nums[i - 1] + nums[j];
        }

        let max1 = Self::find_max(&sums);
        let max2 = Self::find_max_2(&sums, max1, k);
        let max3 = Self::find_max_3(&sums, max2, k);

        let acc = max3.last().unwrap();
        acc.indicies.iter().map(|&i| i as i32).collect()
    }

    fn find_max_3(nums: &Vec<i32>, max2: Vec<Acc2>, k: usize) -> Vec<Acc3> {
        let mut accs: Vec<Acc3> = Vec::new();
        accs.reserve_exact(nums.len() - 2 * k);
        accs.push(Acc3 {
            indicies: [0, k, 2 * k],
            sum: nums[0] + nums[k] + nums[2 * k],
        });
        for i in 1..nums.len() - 2 * k {
            let mut new_acc = accs[i - 1].clone();
            let j = i + 2 * k;
            let new_sum = max2[i].sum + nums[j];
            let new_indicies = [max2[i].indicies[0], max2[i].indicies[1], i + 2 * k];
            if new_sum > new_acc.sum {
                // TODO 字典序
                new_acc.sum = new_sum;
                new_acc.indicies = new_indicies;
            }
            accs.push(new_acc);
        }
        accs
    }

    fn find_max_2(nums: &Vec<i32>, max1: Vec<usize>, k: usize) -> Vec<Acc2> {
        let mut accs: Vec<Acc2> = Vec::new();
        accs.reserve_exact(nums.len() - k);
        accs.push(Acc2 {
            indicies: [0, k],
            sum: nums[0] + nums[k],
        });
        for i in 1..nums.len() - k {
            let mut new_acc = accs[i - 1].clone();
            let new_sum = nums[max1[i]] + nums[i + k];
            if new_sum > new_acc.sum || new_sum == new_acc.sum && max1[i] < new_acc.indicies[0] {
                new_acc.sum = new_sum;
                new_acc.indicies = [max1[i], i + k];
            }
            accs.push(new_acc);
        }
        accs
    }

    fn find_max(nums: &Vec<i32>) -> Vec<usize> {
        let mut accs: Vec<usize> = vec![0usize; nums.len()];
        accs[0] = 0;
        for i in 1..nums.len() {
            accs[i] = if nums[i] > nums[accs[i - 1]] {
                i
            } else {
                accs[i - 1]
            };
        }
        accs
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(vec![1,2,1,2,6,7,5,1], 2 => vec![0, 3, 5])]
    fn test(nums: Vec<i32>, k: i32) -> Vec<i32> {
        Solution::max_sum_of_three_subarrays(nums, k)
    }
}
