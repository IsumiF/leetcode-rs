/*
 * @lc app=leetcode id=682 lang=rust
 *
 * [682] Baseball Game
 */

struct Solution {}

// @lc code=start
impl Solution {
    pub fn cal_points(ops: Vec<String>) -> i32 {
        let mut records = Vec::<i32>::new();
        for op in ops {
            match op.parse() {
                Ok(v) => records.push(v),
                Err(_) => {
                    if op == "C" {
                        records.pop();
                    } else if op == "D" {
                        records.push(*records.last().unwrap() * 2);
                    } else if op == "+" {
                        let len = records.len();
                        records.push(records[len - 2] + records[len - 1]);
                    }
                }
            }
        }
        records.into_iter().sum()
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(vec!["5", "2", "C", "D", "+"] => 30)]
    #[test_case(vec!["5","-2","4","C","D","9","+","+"] => 27)]
    fn test(ops: Vec<&str>) -> i32 {
        Solution::cal_points(ops.into_iter().map(String::from).collect())
    }
}
