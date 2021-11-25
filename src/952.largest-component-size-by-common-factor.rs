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
impl Solution {
    pub fn largest_component_size(nums: Vec<i32>) -> i32 {
        todo!()
    }
}

const SIZE: usize = 100000;

struct PrimeNumberFinder {
    is_prime_arr: [bool; SIZE],
}

impl PrimeNumberFinder {
    fn new() -> Self {
        let is_prime_arr: [bool; SIZE] = [true; SIZE];
        let mut result = Self { is_prime_arr };
        result.init();
        result
    }

    fn init(&mut self) {
        let starting_at: usize = 2;
        let p: usize = 2;
        for i in 1..SIZE / p {
            
        }
    }

    pub fn is_prime(&self, x: usize) -> bool {
        self.is_prime_arr[x - 2]
    }
}

// @lc code=end
