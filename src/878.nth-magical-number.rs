/*
 * @lc app=leetcode id=878 lang=rust
 *
 * [878] Nth Magical Number
 */

struct Solution {}

// @lc code=start
const MODULO: u64 = 1000000007;

fn gcd(a: u64, b: u64) -> u64 {
    if a == 0 {
        b
    } else {
        gcd(b % a, a)
    }
}

fn lcm(a: u64, b: u64) -> u64 {
    a * b / gcd(a, b)
}

impl Solution {
    pub fn nth_magical_number(n: i32, a: i32, b: i32) -> i32 {
        Self::nth_magical_number_impl(n as u64, a as u64, b as u64) as i32
    }

    fn nth_magical_number_impl(n: u64, a: u64, b: u64) -> u64 {
        let c = lcm(a, b);
        // println!("c: {}", c);
        let step = (c / a + c / b) - 1;
        // println!("step: {}", step);
        // c is the 'step'th number divisible by either a or b
        // c * (n / step) is the '(n/step)*step'th number divisible by either a or b
        let mut num = c * (n / step);
        let num_base = num;
        // println!("num: {}", num);
        let mut a_m: u64 = 1;
        let mut b_m: u64 = 1;
        for _ in 0..(n % step) {
            let a_num = num_base + a * a_m;
            let b_num = num_base + b * b_m;
            if a_num < b_num {
                num = a_num;
                a_m += 1;
            } else if a_num > b_num {
                num = b_num;
                b_m += 1;
            } else {
                panic!(
                    "this should not happen: a_num = {}, b_num = {}",
                    a_num, b_num
                );
            }
            // println!("num: {}", num);
            // println!("a_num, b_num, a_m, b_m: {} {} {} {}", a_num, b_num, a_m, b_m);
        }
        num % MODULO
    }

    /*
        int gcd(int a, int b)
    {
        if (a == 0)
            return b;
        return gcd(b % a, a);
    }
        */
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(1, 2, 3 => 2)]
    #[test_case(4, 2, 3 => 6)]
    #[test_case(5, 2, 4 => 10)]
    #[test_case(3, 6, 4 => 8)]
    fn test(n: i32, a: i32, b: i32) -> i32 {
        Solution::nth_magical_number(n, a, b)
    }
}
