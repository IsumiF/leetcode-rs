/*
 * @lc app=leetcode id=902 lang=rust
 *
 * [902] Numbers At Most N Given Digit Set
 */

struct Solution {}

// @lc code=start
impl Solution {
    pub fn at_most_n_given_digit_set(digits: Vec<String>, n: i32) -> i32 {
        Self::solve(
            digits
                .into_iter()
                .map(|s| str::parse(&s).unwrap())
                .collect(),
            n,
        )
    }

    fn solve(mut digits: Vec<i32>, mut n: i32) -> i32 {
        digits.sort();
        let mut total = 0;
        let mut num_ignore_limit: i32 = 0;
        let mut num: i32 = 1;
        while n > 0 {
            total += num_ignore_limit;
            let digit = n % 10;
            let (less, equal) = match digits.binary_search(&digit) {
                Ok(idx) => (idx as i32, 1),
                Err(idx) => (idx as i32, 0),
            };
            if num_ignore_limit == 0 {
                num_ignore_limit = 1;
            }
            let new_num_ignore_limit = (digits.len() as i32) * num_ignore_limit;
            let new_num = less * num_ignore_limit + equal * num;
            num = new_num;
            num_ignore_limit = new_num_ignore_limit;
            // println!(
            //     "num: {}, num_ignore_limit: {}, digit: {}",
            //     num, num_ignore_limit, digit
            // );
            n = n / 10;
        }
        total += num;
        total
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(vec![1, 3, 5, 7], 100 => 20)]
    #[test_case(vec![1, 4, 9], 1000000000 => 29523)]
    #[test_case(vec![7], 8 => 1)]
    #[test_case(vec![7], 6 => 0)]
    #[test_case(vec![7], 16 => 1)]
    #[test_case(vec![3, 4, 8], 4 => 2)]
    fn test(digits: Vec<i32>, n: i32) -> i32 {
        Solution::at_most_n_given_digit_set(digits.into_iter().map(|d| d.to_string()).collect(), n)
    }
}
