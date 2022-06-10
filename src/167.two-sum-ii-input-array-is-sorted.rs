/*
 * @lc app=leetcode id=167 lang=rust
 *
 * [167] Two Sum II - Input Array Is Sorted
 */

struct Solution {}

// @lc code=start
impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        for (i, &num) in numbers.iter().enumerate() {
            let rem = target - num;
            match numbers.binary_search(&rem) {
                Ok(j) => {
                    if j != i {
                        let left = i.min(j) as i32;
                        let right = i.max(j) as i32;
                        return vec![left + 1, right + 1];
                    }
                }
                Err(_) => {}
            }
        }
        panic!("not found")
    }
}
// @lc code=end

#[cfg(test)]
mod tests {

    fn test() {}
}
