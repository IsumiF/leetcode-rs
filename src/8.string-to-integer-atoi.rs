/*
 * @lc app=leetcode id=8 lang=rust
 *
 * [8] String to Integer (atoi)
 */

struct Solution {}

// @lc code=start
impl Solution {
    pub fn my_atoi(s: String) -> i32 {
        let mut chars = s.chars().skip_while(|ch| ch.is_whitespace()).peekable();
        let first = chars.peek();
        let sign = match first {
            None => return 0,
            Some(&ch) => {
                if ch == '+' {
                    chars.next();
                    1i32
                } else if ch == '-' {
                    chars.next();
                    -1i32
                } else {
                    1i32
                }
            }
        };
        let mut value: Option<i32> = Some(0);
        for ch in chars {
            match ch.to_digit(10) {
                None => break,
                Some(d) => {
                    value = value.and_then(|v| {
                        v.checked_mul(10)
                            .and_then(|v| v.checked_add(d as i32 * sign))
                    });
                }
            }
        }
        value.unwrap_or(if sign > 0 { i32::MAX } else { i32::MIN })
    }
}
// @lc code=end

#[cfg(test)]
mod tests{
    use super::*;
    use test_case::test_case;

    #[test_case("  -42" => -42)]
    fn test(s: &str) -> i32{
        Solution::my_atoi(s.to_string())
    }
}