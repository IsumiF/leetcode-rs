/*
 * @lc app=leetcode id=1094 lang=rust
 *
 * [1094] Car Pooling
 */

struct Solution {}

// @lc code=start
use std::collections::HashMap;

impl Solution {
    pub fn car_pooling(trips: Vec<Vec<i32>>, capacity: i32) -> bool {
        let mut loc_to_num = HashMap::<i32, i32>::new();
        trips
            .into_iter()
            .for_each(|trip| {
                let num_passengers = trip[0];
                let from = trip[1];
                let to = trip[2];
                let new_from_num = loc_to_num.remove(&from).unwrap_or(0) + num_passengers;
                loc_to_num.insert(from, new_from_num);
                let new_to_num = loc_to_num.remove(&to).unwrap_or(0) - num_passengers;
                loc_to_num.insert(to, new_to_num);
            });
        let mut actions: Vec<(i32, i32)> = loc_to_num.into_iter().collect();
        actions.sort_by_key(|act| act.0);
        let mut size = 0;
        for (_, num) in actions {
            size += num;
            if size > capacity || size < 0 {
                return false;
            }
        }
        true
    }
}

// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(vec![[4,5,6],[6,4,7],[4,3,5],[2,3,5]], 13 => true)]
    fn test(trips: Vec<[i32; 3]>, capacity: i32) -> bool {
        Solution::car_pooling(trips.into_iter().map(|v| Vec::from(v)).collect(), capacity)
    }
}
