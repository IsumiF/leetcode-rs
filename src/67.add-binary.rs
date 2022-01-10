/*
 * @lc app=leetcode id=67 lang=rust
 *
 * [67] Add Binary
 */

struct Solution {}

// @lc code=start
impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        let mut a_chs = a.chars().rev();
        let mut b_chs = b.chars().rev();
        let mut c = 0;
        let mut result = Vec::<char>::new();
        result.reserve_exact(std::cmp::max(a.len(), b.len()) + 2);
        loop {
            let a = a_chs.next();
            let b = b_chs.next();
            if a.is_none() && b.is_none() {
                break;
            }
            let a = char_to_digit(a.unwrap_or('0'));
            let b = char_to_digit(b.unwrap_or('0'));
            let mut x = a + b + c;
            c = if x >= 2 {
                x -= 2;
                1
            } else {
                0
            };
            result.push(digit_to_char(x));
        }
        if c > 0 {
            result.push(digit_to_char(c));
        }
        result.into_iter().rev().collect::<String>()
    }
}

fn char_to_digit(ch: char) -> i32 {
    match ch {
        '0' => 0,
        '1' => 1,
        _ => panic!("only 0 and 1 is supported"),
    }
}

fn digit_to_char(d: i32) -> char {
    match d {
        0 => '0',
        1 => '1',
        _ => panic!("only 0 and 1 is supported"),
    }
}

// @lc code=end
